/* window.rs
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

use adw::subclass::prelude::*;
use gtk::{
    gio, glib,
    glib::{clone, MainContext},
    prelude::*,
};

use crate::{
    discover::view::DiscoverView, empty_view::EmptyView, podcasts,
    podcasts_view_stack::PodcastsViewStack,
};

pub enum View {
    Empty,
    Loading,
    Loaded,
    Discover, // PodcastView
}

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/kylobytes/Bolt/gtk/window.ui")]
    pub struct BoltWindow {
        // Template widgets
        #[template_child]
        pub main_stack: TemplateChild<adw::ViewStack>,
        #[template_child]
        pub discover_view: TemplateChild<DiscoverView>,
        #[template_child]
        pub podcasts_view_stack: TemplateChild<PodcastsViewStack>,
        #[template_child]
        pub empty_view: TemplateChild<EmptyView>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BoltWindow {
        const NAME: &'static str = "BoltWindow";
        type Type = super::BoltWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for BoltWindow {}
    impl WidgetImpl for BoltWindow {}
    impl WindowImpl for BoltWindow {}
    impl ApplicationWindowImpl for BoltWindow {}
    impl AdwApplicationWindowImpl for BoltWindow {}
}

glib::wrapper! {
    pub struct BoltWindow(ObjectSubclass<imp::BoltWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
    @implements gio::ActionGroup, gio::ActionMap;
}

impl BoltWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        let window = glib::Object::builder::<BoltWindow>()
            .property("application", application)
            .build();

        window.connect_signals();
        window.load_shows();

        window
    }

    pub fn show_view(&self, view: View) {
        let stack = self.imp().main_stack.get();

        match view {
            View::Empty => stack.set_visible_child_name("empty-view"),
            View::Loading => stack.set_visible_child_name("loading-view"),
            View::Loaded => stack.set_visible_child_name("podcasts-view"),
            View::Discover => {
                stack.set_visible_child_name("discover-view");
            }
        };
    }

    fn load_shows(&self) {
        let main_context = MainContext::default();

        self.show_view(View::Loading);

        main_context.spawn_local(clone!(@weak self as window => async move {
            if let Ok(shows) = podcasts::repository::load_all_shows() {
                if shows.is_empty() {
                    window.show_view(View::Empty);
                }
            }
        }));
    }

    fn connect_signals(&self) {
        self.imp().empty_view.btn_discover().connect_clicked(
            clone!(@weak self as window => move |_| {
                window.show_view(View::Discover);
            }),
        );

        self.imp()
            .podcasts_view_stack
            .btn_discover()
            .connect_clicked(clone!(@weak self as window => move |_| {
                window.show_view(View::Discover);
            }));

        let discover_view = self.imp().discover_view.get();
        let discover_search_entry = discover_view.search_entry();

        discover_search_entry.connect_search_changed(
            move |entry: &gtk::SearchEntry| {
                if entry.text().len() > 3 {
                    discover_view.search_shows(&entry.text());
                }
            },
        );
    }
}
