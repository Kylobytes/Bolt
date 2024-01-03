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
    card::DiscoverCard, repository::DiscoverRepository,
    search_result::SearchResult,
};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/kylobytes/Bolt/gtk/discover-view.ui")]
    pub struct DiscoverView {
        #[template_child]
        pub episodes_spinner: TemplateChild<gtk::Spinner>,
        #[template_child]
        pub episodes_container: TemplateChild<gtk::FlowBox>,
        #[template_child]
        pub categories_spinner: TemplateChild<gtk::Spinner>,
        #[template_child]
        pub categories_container: TemplateChild<gtk::FlowBox>,
        #[template_child]
        pub search_bar: TemplateChild<gtk::SearchBar>,
        #[template_child]
        pub search_entry: TemplateChild<gtk::SearchEntry>,
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
                .replace(Some(gio::ListStore::new::<SearchResult>()));

            let model_binding = self.model.borrow();
            let model = model_binding.as_ref();

            self.search_results_container.get().bind_model(
                model,
                |item: &glib::Object| {
                    let episode = item
                        .downcast_ref::<SearchResult>()
                        .expect("Item must be an episode");

                    let card = DiscoverCard::from(episode.to_owned());

                    glib::spawn_future_local(
                        clone!(@weak card, @weak episode => async move {
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

    pub fn show_front_page(&self) {
        glib::spawn_future_local(clone!(@weak self as view => async move {
            let episodes: Vec<DiscoverEpisode> = gio::spawn_blocking(move || {
                DiscoverRepository::fetch_recent_episodes()
                    .expect("Failed to fetch latest episodes")
            }).await.expect("Failed to fetch episodes on separate thread")
                .into_iter()
                .map(DiscoverEpisode::new)
                .collect();

            if let Some(model) = view.imp().model.borrow().as_ref() {
                model.extend_from_slice(&episodes);

                view.imp().episodes_spinner.get().stop();
                view.imp().episodes_spinner.get().set_visible(false);
                view.imp().categories_spinner.get().stop();
                view.imp().categories_spinner.get().set_visible(false);
                view.imp().episodes_container.get().set_visible(true);
                view.imp().categories_container.get().set_visible(true);
            };
        }));
    }

    pub fn search_entry(&self) -> gtk::SearchEntry {
        self.imp().search_entry.get()
    }

    pub fn search_bar(&self) -> gtk::SearchBar {
        self.imp().search_bar.get()
    }
}
