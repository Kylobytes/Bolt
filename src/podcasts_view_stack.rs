/* podcasts_view_stack.rs
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
 *
 */

use adw::subclass::prelude::*;
use gtk::{gio, glib};

use crate::queue_view::QueueView;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/kylobytes/Bolt/gtk/podcasts-view-stack.ui")]
    pub struct PodcastsViewStack {
        #[template_child]
        queue_view: TemplateChild<QueueView>,
        #[template_child]
        pub btn_discover: TemplateChild<gtk::Button>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PodcastsViewStack {
        const NAME: &'static str = "PodcastsViewStack";
        type Type = super::PodcastsViewStack;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for PodcastsViewStack {}
    impl WidgetImpl for PodcastsViewStack {}
    impl BinImpl for PodcastsViewStack {}
}

glib::wrapper! {
    pub struct PodcastsViewStack(ObjectSubclass<imp::PodcastsViewStack>)
        @extends gtk::Widget, adw::Bin,
    @implements gio::ActionGroup, gio::ActionMap;
}

impl Default for PodcastsViewStack {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl PodcastsViewStack {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn btn_discover(&self) -> gtk::Button {
        self.imp().btn_discover.get()
    }
}
