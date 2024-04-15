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
    gio,
    glib::{self, clone},
    prelude::*,
};

use crate::{
    api::search::result::SearchResult,
    data::podcast::{self, Podcast},
    explore::card::ExploreCard,
    runtime,
};

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
        pub model: RefCell<Option<gio::ListStore>>,
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

            let model = gio::ListStore::new::<ExploreCard>();
            self.model.replace(Some(model.clone()));

            self.search_results.bind_model(Some(&model), move |item| {
                item.downcast_ref::<ExploreCard>()
                    .expect("Item needs to be an explore card")
                    .to_owned()
                    .into()
            });
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

    pub fn search_result_at_index(&self, index: &i32) -> Option<Podcast> {
        let Ok(index) = u32::try_from(index.clone()) else {
            return None;
        };

        let Some(ref model) = *self.imp().model.borrow() else {
            return None;
        };

        let Some(object) = model.item(index) else {
            return None;
        };

        // if let Ok(data) = object.downcast::<Podcast>() {
        //     Some(data)
        // } else {
        //     None
        // }

        None
    }

    pub fn load_search_results(&self, query: &str) {
        let imp = self.imp();

        imp.welcome.get().set_visible(false);
        imp.explore_spinner.get().set_visible(true);
        imp.results_empty.get().set_visible(false);
        imp.search_results.get().set_visible(false);

        let (sender, receiver) =
            async_channel::bounded::<Vec<SearchResult>>(1);

        let query = query.to_string();

        runtime().spawn(clone!(@strong query, @strong sender => async move {
            let podcasts: Vec<SearchResult> =
            podcast::repository::search(&query).await;

            sender
                .send(podcasts)
                .await
                .expect("The search channel must be open");
        }));

        glib::spawn_future_local(
            clone!(@strong receiver, @weak imp => async move {
                while let Ok(podcasts) = receiver.recv().await {
                    if let Some(ref model) = *imp.model.borrow() {
                        let cards: Vec<ExploreCard> = podcasts.iter()
                        .map(|podcast| ExploreCard::from(podcast.clone()))
                        .collect();

                        model.remove_all();
                        model.extend_from_slice(&cards);
                        imp.explore_spinner.get().set_visible(false);
                        imp.search_results.set_visible(true);
                    }
                }
            }),
        );
    }
}
