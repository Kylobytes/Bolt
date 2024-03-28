/* view.rs
 *
 * Copyright 2023 Kent Delante
 *
 * This file is part of Bolt.
 *
 * Bolt is free software: you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Bolt is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Bolt. If not, see <https://www.gnu.org/licenses/>.
 */

use std::cell::RefCell;

use adw::subclass::prelude::*;
use gtk::{
    gio::{self, ListStore},
    glib::{self, clone},
    prelude::*,
};

use crate::{
    api::{podcasts, search::response::SearchResponse},
    data::podcast,
    runtime,
};

use super::{card::ExploreCard, card_data::CardData};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/kylobytes/Bolt/gtk/explore/view.ui")]
    pub struct ExploreView {
        #[template_child]
        pub back_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub search_entry: TemplateChild<gtk::SearchEntry>,
        #[template_child]
        pub welcome: TemplateChild<adw::StatusPage>,
        #[template_child]
        pub search_results: TemplateChild<gtk::FlowBox>,
        #[template_child]
        pub categories: TemplateChild<gtk::FlowBox>,
        #[template_child]
        pub results_empty: TemplateChild<adw::StatusPage>,
        #[template_child]
        pub explore_spinner: TemplateChild<gtk::Spinner>,
        pub model: RefCell<Option<ListStore>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ExploreView {
        const NAME: &'static str = "ExploreView";
        type Type = super::ExploreView;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ExploreView {
        fn constructed(&self) {
            self.parent_constructed();

            let model = gio::ListStore::new::<CardData>();
            self.model.replace(Some(model));

            if let Some(ref model) = *self.model.borrow() {
                self.search_results.bind_model(Some(model), move |object| {
                    let data = object.downcast_ref::<CardData>().unwrap();

                    let card = ExploreCard::new();
                    card.set_name(&data.name());
                    card.set_description(&data.description());

                    if !data.image_url().is_empty() {
                        card.load_image(&data.id(), &data.image_url());
                    }

                    if data.subscribed() {
                        card.subscribe_button().set_visible(false);
                        card.unsubscribe_button().set_visible(true);
                    }

                    let id = data.id().clone();

                    card.subscribe_button().connect_clicked(
                        clone!(@weak card, @strong id => move |_| {
                            card.subscribe(&id);
                        }),
                    );

                    card.unsubscribe_button().connect_clicked(
                        clone!(@weak card, @strong id => move |_| {
                            card.unsubscribe(&id);
                        }),
                    );

                    card.into()
                });
            }
        }
    }

    impl WidgetImpl for ExploreView {}
    impl BinImpl for ExploreView {}
}

glib::wrapper! {
    pub struct ExploreView(ObjectSubclass<imp::ExploreView>)
        @extends gtk::Widget, adw::Bin,
    @implements gio::ActionGroup, gio::ActionMap;
}

impl Default for ExploreView {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl ExploreView {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn back_button(&self) -> gtk::Button {
        self.imp().back_button.get()
    }

    pub fn search_entry(&self) -> gtk::SearchEntry {
        self.imp().search_entry.get()
    }

    pub fn search_results(&self) -> gtk::FlowBox {
        self.imp().search_results.get()
    }

    pub fn search_result_at_index(&self, index: &i32) -> Option<CardData> {
        if let Some(ref model) = *self.imp().model.borrow() {
            let Some(object) = model
                .item(index.clone().try_into().expect("Failed to cast index"))
            else {
                return None;
            };

            if let Ok(data) = object.downcast::<CardData>() {
                Some(data)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn load_search_results(&self, query: &str) {
        self.imp().results_empty.get().set_visible(false);
        self.imp().welcome.get().set_visible(false);
        self.imp().explore_spinner.get().set_visible(true);

        let query = query.to_string();

        let (sender, receiver) = async_channel::bounded(1);

        runtime().spawn(clone!(@strong query, @strong sender => async move {
            let response: SearchResponse = podcasts::search(&query).await;
            let remote_ids = response.feeds.clone().into_iter().map(|feed| feed.id).collect();
            let subscribed_ids = podcast::repository::load_subscribed_ids(&remote_ids).await;

            sender.send((response.feeds, subscribed_ids)).await.unwrap();
        }));

        glib::spawn_future_local(
            clone!(@weak self as view, @strong receiver => async move {
                while let Ok(results_and_ids) = receiver.recv().await {
                    let (search_results, subscribed_ids) = results_and_ids;
                    view.imp().explore_spinner.get().set_visible(false);

                    if search_results.is_empty() {
                        view.imp().search_results.set_visible(false);
                        view.imp().results_empty.get().set_visible(true);

                        return;
                    }

                    if let Some(ref model) = *view.imp().model.borrow() {
                        let card_data: Vec<CardData> = search_results
                            .into_iter()
                            .map(|result| {
                                let id = result.id;
                                let data = CardData::from(result);

                                if subscribed_ids.contains(&id) {
                                    data.set_subscribed(true);
                                }

                                data
                            })
                            .collect();

                        model.remove_all();
                        model.extend_from_slice(&card_data);

                        view.imp().search_results.set_visible(true);
                    }
                }
            }),
        );
    }
}
