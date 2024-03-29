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
 *
 * You should have received a copy of the GNU General Public License
 * along with Bolt. If not, see <https://www.gnu.org/licenses/>.
 *
 */

use adw::subclass::prelude::*;
use gtk::{gio, glib};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/kylobytes/Bolt/gtk/empty/view.ui")]
    pub struct EmptyView {
        #[template_child]
        pub btn_explore: TemplateChild<gtk::Button>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for EmptyView {
        const NAME: &'static str = "EmptyView";
        type Type = super::EmptyView;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for EmptyView {}
    impl WidgetImpl for EmptyView {}
    impl BinImpl for EmptyView {}
}

glib::wrapper! {
    pub struct EmptyView(ObjectSubclass<imp::EmptyView>)
        @extends gtk::Widget, adw::Bin,
    @implements gio::ActionGroup, gio::ActionMap;
}

impl Default for EmptyView {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl EmptyView {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn btn_explore(&self) -> gtk::Button {
        self.imp().btn_explore.get()
    }
}
