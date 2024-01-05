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
 *https://api.podcastindex.org/api/1.0/recent/feeds?pretty
 * You should have received a copy of the GNU General Public License
 * along with Bolt. If not, see <https://www.gnu.org/licenses/>.
 *
 */

use std::cell::RefCell;

use adw::subclass::prelude::*;
use gtk::{
    gio,
    glib::{self, clone},
    prelude::*,
};

use crate::discover::{
    card::DiscoverCard, repository::DiscoverRepository, show::DiscoverShow,
};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/kylobytes/Bolt/gtk/discover-view.ui")]
    pub struct DiscoverView {
        #[template_child]
        pub search_entry: TemplateChild<gtk::SearchEntry>,
        #[template_child]
        pub discover_welcome: TemplateChild<gtk::Label>,
        #[template_child]
        pub search_results_container: TemplateChild<gtk::FlowBox>,
        #[template_child]
        pub categories_container: TemplateChild<gtk::FlowBox>,
        #[template_child]
        pub discover_results_empty: TemplateChild<gtk::Label>,
        #[template_child]
        pub discover_spinner: TemplateChild<gtk::Spinner>,
        pub model: RefCell<Option<gio::ListStore>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DiscoverView {
        const NAME: &'static str = "DiscoverView";
        type Type = super::DiscoverView;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for DiscoverView {
        fn constructed(&self) {
            self.parent_constructed();
            self.model
                .replace(Some(gio::ListStore::new::<DiscoverShow>()));

            let model_binding = self.model.borrow();
            let model = model_binding.as_ref();

            self.search_results_container.get().bind_model(
                model,
                |item: &glib::Object| {
                    let show = item
                        .downcast_ref::<DiscoverShow>()
                        .expect("Item must be an search result");

                    let card = DiscoverCard::from(show.to_owned());

                    glib::spawn_future_local(
                        clone!(@weak card => async move {
                            card.load_image().await;
                        }),
                    );

                    card.into()
                },
            );
        }
    }
    impl WidgetImpl for DiscoverView {}
    impl BinImpl for DiscoverView {}
}

glib::wrapper! {
    pub struct DiscoverView(ObjectSubclass<imp::DiscoverView>)
        @extends gtk::Widget, adw::Bin,
    @implements gio::ActionGroup, gio::ActionMap;
}

impl Default for DiscoverView {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl DiscoverView {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn search_entry(&self) -> gtk::SearchEntry {
        self.imp().search_entry.get()
    }

    pub fn search_shows(&self, query: &str) {
        self.imp().discover_welcome.get().set_visible(false);

        let spinner = self.imp().discover_spinner.get();
        spinner.start();
        spinner.set_visible(true);

        let shows = DiscoverRepository::search_shows(&query);
        let discover_shows: Vec<DiscoverShow> =
            shows.into_iter().map(DiscoverShow::from).collect();

        let model_binding = self.imp().model.borrow();
        let model = model_binding.as_ref();

        if let Some(model) = model {
            model.remove_all();
            model.extend_from_slice(&discover_shows);
        }

        spinner.stop();
        spinner.set_visible(false);

        self.imp().search_results_container.get().set_visible(true);
    }
}
