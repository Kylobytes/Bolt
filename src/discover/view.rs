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
    glib::{self, clone, subclass::Signal},
    prelude::*,
};
use once_cell::sync::Lazy;

use crate::{
    data::show::object::ShowObject,
    discover::{self, card::DiscoverCard},
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
        pub discover_welcome: TemplateChild<gtk::Label>,
        #[template_child]
        pub search_results_container: TemplateChild<gtk::FlowBox>,
        #[template_child]
        pub categories_container: TemplateChild<gtk::FlowBox>,
        #[template_child]
        pub discover_results_empty: TemplateChild<adw::StatusPage>,
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
                .replace(Some(gio::ListStore::new::<ShowObject>()));

            let model_binding = self.model.borrow();
            let model = model_binding.as_ref();

            self.search_results_container.get().bind_model(
                model,
                move |item: &glib::Object| {
                    let show = item
                        .downcast_ref::<ShowObject>()
                        .expect("Item must be an search result");

                    DiscoverCard::from(show.to_owned()).into()
                },
            );

            self.obj().connect_signals();
        }

        fn signals() -> &'static [Signal] {
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
                vec![Signal::builder("search-result-activated")
                    .param_types([ShowObject::static_type()])
                    .build()]
            });

            SIGNALS.as_ref()
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

    pub fn back_button(&self) -> gtk::Button {
        self.imp().back_button.get()
    }

    pub fn search_shows(&self, search_query: &str) {
        let query = search_query.to_string();

        self.imp().discover_results_empty.set_visible(false);

        glib::spawn_future_local(
            clone!(@weak self as view, @strong query => async move {
                view.imp().discover_welcome.get().set_visible(false);

                let model_binding = view.imp().model.borrow();
                let model = model_binding.as_ref();

                if let Some(model) = model {
                    model.remove_all();
                }

                let spinner = view.imp().discover_spinner.get();
                spinner.start();
                spinner.set_visible(true);

                let search_results = gio::spawn_blocking(move || {
                    discover::repository::search_shows(&query)
                }).await.expect("Failed to complete show search");

                let subscribed_shows = gio::spawn_blocking(move || {
                    discover::repository::load_subscribed_show_ids()
                }).await.expect("Failed to complete show loading task");

                let discover_shows: Vec<ShowObject> = search_results
                    .into_iter()
                    .map(|search_result| {
                        let show = ShowObject::from(search_result);

                        if subscribed_shows.contains(&show.id()) {
                            show.mark_subscribed();
                        }

                        show
                    })
                    .collect();


                if let Some(model) = model {
                    model.extend_from_slice(&discover_shows);
                }

                spinner.stop();
                spinner.set_visible(false);

                if discover_shows.len() > 0 {
                    view.imp().search_results_container.get().set_visible(true);
                } else {
                    view.imp().discover_results_empty.get().set_visible(true);
                }
            }),
        );
    }

    pub fn connect_signals(&self) {
        self.imp()
            .search_results_container
            .get()
            .connect_child_activated(
                clone!(@weak self as view => move |_container, child| {
                    if let Some(ref model) = *view.imp().model.borrow() {
                        let index: u32 = child.index().try_into().expect("Index cannot be out of range");
                        let show = model.item(index);
                        view.emit_by_name::<()>("search-result-activated", &[&show]);
                    };
                }),
            );
    }
}
