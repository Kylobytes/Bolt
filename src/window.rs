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
use gtk::prelude::ButtonExt;
use gtk::{gio, glib};
use gtk::glib::{MainContext, clone};

use crate::discover_view::DiscoverView;
use crate::podcasts_view_stack::PodcastsViewStack;
use crate::empty_view::EmptyView;
use crate::data::repository::show_repository;

pub enum WindowState {
    Empty,
    Loading,
    Loaded,
    Discover
    // PodcastView
}

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/kylobytes/Bolt/gtk/window.ui")]
    pub struct BoltWindow {
        // Template widgets
        #[template_child]
        pub main_stack: TemplateChild<gtk::Stack>,
        #[template_child]
        discover_view: TemplateChild<DiscoverView>,
        #[template_child]
        podcasts_view_stack: TemplateChild<PodcastsViewStack>,
        #[template_child]
        pub empty_view: TemplateChild<EmptyView>
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

    pub fn set_state(&self, view: WindowState) {
        let stack = self.imp().main_stack.get();

        match view {
            WindowState::Empty => {
                stack.set_visible_child_name("empty-page")
            },
            WindowState::Loading => {
                stack.set_visible_child_name("loading-page")
            },
            WindowState::Loaded => {
                stack.set_visible_child_name("podcasts-page")
            },
            WindowState::Discover => {
                stack.set_visible_child_name("discover-page")
            },
        };
    }    

    fn load_shows(&self) {
        let main_context = MainContext::default();

        self.set_state(WindowState::Loading);

        main_context.spawn_local(clone!(@weak self as window => async move {
            let shows_result = show_repository::load_all_shows();

            if let Ok(shows) = shows_result {
                if shows.is_empty() {
                    window.set_state(WindowState::Empty);
                }
            }
        }));
    }

    fn connect_signals(&self) {
        self.imp().empty_view.btn_discover().connect_clicked(
            clone!(@weak self as window => move |_| {
                window.set_state(WindowState::Discover);
            })
        );
    }
}
