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
use gtk::{gio, glib};

use crate::{
    empty::view::EmptyView,
    episodes::view::EpisodesView,
    explore::{preview::Preview, view::ExploreView},
    queue_view::QueueView,
    show_details::view::ShowDetails,
};

#[derive(Clone, Debug)]
pub enum View {
    Loading,
    Empty,
    Explore,
    Podcasts,
    Preview,
    ShowDetails,
}

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/kylobytes/Bolt/gtk/window.ui")]
    pub struct BoltWindow {
        #[template_child]
        pub main_stack: TemplateChild<gtk::Stack>,
        #[template_child]
        pub explore_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub refresh_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub explore_view: TemplateChild<ExploreView>,
        #[template_child]
        pub empty_view: TemplateChild<EmptyView>,
        #[template_child]
        pub queue_view: TemplateChild<QueueView>,
        #[template_child]
        pub episodes_view: TemplateChild<EpisodesView>,
        #[template_child]
        pub show_details_view: TemplateChild<ShowDetails>,
        #[template_child]
        pub preview: TemplateChild<Preview>,
        #[template_child]
        pub podcasts_stack: TemplateChild<adw::ViewStack>,
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

        window.initialize();
        window.connect_signals();
        window.setup_explore();
        window.setup_episodes();

        window
    }

    fn initialize(&self) {
        self.switch_view(View::Loading);
    }

    fn switch_view(&self, view: View) {
        let stack = self.imp().main_stack.get();

        match view {
            View::Loading => stack.set_visible_child_name("loading-view"),
            View::Empty => stack.set_visible_child_name("empty-view"),
            View::Explore => stack.set_visible_child_name("explore-view"),
            View::Podcasts => stack.set_visible_child_name("podcasts-view"),
            View::Preview => stack.set_visible_child_name("preview"),
            View::ShowDetails => {
                stack.set_visible_child_name("show-details-view")
            }
        }
    }

    fn connect_signals(&self) {}
    fn setup_explore(&self) {}
    fn return_from_explore(&self) {}
    fn setup_episodes(&self) {}
}
