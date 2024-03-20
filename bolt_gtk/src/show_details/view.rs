/* view.rs
 *
 * Copyright 2024 Kent Delante
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

use std::cell::RefCell;

use adw::subclass::prelude::*;
use gtk::{gio, glib};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/kylobytes/Bolt/gtk/show-details/view.ui")]
    pub struct ShowDetails {
        #[template_child]
        pub back_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub title: TemplateChild<gtk::Label>,
        #[template_child]
        pub picture_container: TemplateChild<adw::Clamp>,
        #[template_child]
        pub picture: TemplateChild<gtk::Picture>,
        #[template_child]
        pub spinner_container: TemplateChild<adw::Clamp>,
        #[template_child]
        pub picture_spinner: TemplateChild<gtk::Spinner>,
        #[template_child]
        pub image_missing_icon: TemplateChild<gtk::Image>,
        #[template_child]
        pub subscribe_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub description: TemplateChild<gtk::TextView>,
        #[template_child]
        pub podcasts: TemplateChild<gtk::ListBox>,
        pub model: RefCell<Option<gio::ListStore>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ShowDetails {
        const NAME: &'static str = "ShowDetails";
        type Type = super::ShowDetails;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ShowDetails {}
    impl WidgetImpl for ShowDetails {}
    impl BinImpl for ShowDetails {}
}

glib::wrapper! {
    pub struct ShowDetails(ObjectSubclass<imp::ShowDetails>)
        @extends gtk::Widget, adw::Bin,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl Default for ShowDetails {
    fn default() -> Self {
        glib::Object::new()
    }
}
