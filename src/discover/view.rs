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
    gio::{self, ListStore},
    glib::{self, clone},
    prelude::*,
};

use crate::{
    api::search::result::SearchResult,
    data::show::object::ShowObject,
    discover::{self, card::DiscoverCard},
    runtime,
};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/kylobytes/Bolt/gtk/discover/view.ui")]
    pub struct DiscoverView {
        #[template_child]
        pub back_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub search_entry: TemplateChild<gtk::SearchEntry>,
        #[template_child]
        pub discover_welcome: TemplateChild<adw::StatusPage>,
        #[template_child]
        pub search_results: TemplateChild<gtk::FlowBox>,
        #[template_child]
        pub categories: TemplateChild<gtk::FlowBox>,
        #[template_child]
        pub discover_results_empty: TemplateChild<adw::StatusPage>,
        #[template_child]
        pub discover_spinner: TemplateChild<gtk::Spinner>,
        pub model: RefCell<Option<ListStore>>,
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

    impl ObjectImpl for DiscoverView {}
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

    pub fn back_button(&self) -> gtk::Button {
        self.imp().back_button.get()
    }

    pub fn search_results(&self) -> gtk::FlowBox {
        self.imp().search_results.get()
    }

    pub fn setup_model(&self, model: &ListStore) {
        self.imp().model.replace(Some(model.clone()));
        self.imp().search_results.get().bind_model(
            Some(model),
            move |item: &glib::Object| {
                let show = item
                    .downcast_ref::<ShowObject>()
                    .expect("Item must be a search result");

                DiscoverCard::from(show.to_owned()).into()
            },
        )
    }

    pub fn search_shows(&self, search_query: &str) {
        let query = search_query.to_string();

        self.imp().discover_results_empty.set_visible(false);

        let (sender, receiver) =
            async_channel::bounded::<(Vec<SearchResult>, Vec<i64>)>(1);

        runtime().spawn(clone!(@strong query => async move {
            let search_results: Vec<SearchResult> = discover::repository::search_shows(&query).await;
            let subscribed_shows: Vec<i64> = discover::repository::load_subscribed_show_ids().await;

            sender.send((search_results, subscribed_shows)).await.expect("The channel must be open");
        }));

        glib::spawn_future_local(
            clone!(@weak self as view, @strong query, @strong receiver => async move {
                let Some(ref model) = *view.imp().model.borrow() else {
                    return
                };

                if let Ok(response) = receiver.recv().await {
                    view.imp().discover_welcome.get().set_visible(false);

                    let spinner = view.imp().discover_spinner.get();
                    spinner.start();
                    spinner.set_visible(true);

                    let (search_results, subscribed_shows) = response;
                    let shows: Vec<ShowObject> = search_results
                        .into_iter()
                        .map(|search_result| {
                            let show = ShowObject::from(search_result);

                            if subscribed_shows.contains(&show.id()) {
                                show.mark_subscribed();
                            }

                            show
                        }).collect();

                    model.remove_all();
                    model.extend_from_slice(&shows);

                    spinner.stop();
                    spinner.set_visible(false);

                    if shows.len() > 0 {
                        view.imp().search_results.get().set_visible(true);
                    } else {
                        view.imp().discover_results_empty.get().set_visible(true);
                    }
                }
            }),
        );
    }
}
