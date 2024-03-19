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

    pub fn load_search_results(&self, query: &str) {
        self.imp().results_empty.get().set_visible(false);
        self.imp().welcome.get().set_visible(false);
        self.imp().explore_spinner.get().set_visible(false);

        let query = query.to_string();

        let (sender, receiver) = async_channel::bounded(1);

        runtime().spawn(clone!(@strong query, @strong sender => async move {
            let response: SearchResponse = podcasts::search(&query).await;

            sender.send(response.feeds).await.unwrap();
        }));

        glib::spawn_future_local(
            clone!(@weak self as view, @strong receiver => async move {
                while let Ok(search_results) = receiver.recv().await {
                    view.imp().explore_spinner.get().set_visible(false);

                    if search_results.is_empty() {
                        view.imp().search_results.set_visible(false);
                        view.imp().results_empty.get().set_visible(true);

                        return;
                    }

                    if let Some(ref model) = *view.imp().model.borrow() {
                        let card_data: Vec<CardData> = search_results
                            .into_iter()
                            .map(CardData::from)
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
