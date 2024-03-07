/* card.rs
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

use std::cell::Cell;

use adw::prelude::*;
use gtk::{gio, glib, subclass::prelude::*};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/kylobytes/Bolt/gtk/discover/card.ui")]
    pub struct DiscoverCard {
        #[template_child]
        pub picture_spinner: TemplateChild<gtk::Spinner>,
        #[template_child]
        pub picture: TemplateChild<gtk::Picture>,
        #[template_child]
        pub image_missing_icon: TemplateChild<gtk::Image>,
        #[template_child]
        pub name: TemplateChild<gtk::Label>,
        #[template_child]
        pub description: TemplateChild<gtk::Label>,
        #[template_child]
        pub subscribe_button: TemplateChild<gtk::Button>,
        pub show_id: Cell<i64>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DiscoverCard {
        const NAME: &'static str = "DiscoverCard";
        type Type = super::DiscoverCard;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for DiscoverCard {}
    impl WidgetImpl for DiscoverCard {}
    impl BoxImpl for DiscoverCard {}
}

glib::wrapper! {
    pub struct DiscoverCard(ObjectSubclass<imp::DiscoverCard>)
        @extends gtk::Widget, gtk::Box,
    @implements gio::ActionGroup, gio::ActionMap;
}

impl Default for DiscoverCard {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl DiscoverCard {
    pub fn new() -> Self {
        Self::default()
    }
}
