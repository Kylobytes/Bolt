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
 */

use std::cell::RefCell;

use adw::subclass::prelude::*;
use gtk::{
    gio::{self, ListStore},
    glib,
    prelude::*,
};

use crate::data::podcast::Podcast;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/kylobytes/Bolt/gtk/explore/view.ui")]
    pub struct ExploreView {
        #[template_child]
        pub back_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub search_entry: TemplateChild<gtk::SearchEntry>,
        #[template_child]
        pub welcome: TemplateChild<adw::StatusPage>,
        #[template_child]
        pub search_results: TemplateChild<gtk::FlowBox>,
        #[template_child]
        pub categories: TemplateChild<gtk::FlowBox>,
        #[template_child]
        pub results_empty: TemplateChild<adw::StatusPage>,
        #[template_child]
        pub explore_spinner: TemplateChild<gtk::Spinner>,
        pub model: RefCell<Option<ListStore>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ExploreView {
        const NAME: &'static str = "ExploreView";
        type Type = super::ExploreView;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ExploreView {}
    impl WidgetImpl for ExploreView {}
    impl BinImpl for ExploreView {}
}

glib::wrapper! {
    pub struct ExploreView(ObjectSubclass<imp::ExploreView>)
        @extends gtk::Widget, adw::Bin,
    @implements gio::ActionGroup, gio::ActionMap;
}

impl Default for ExploreView {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl ExploreView {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn back_button(&self) -> gtk::Button {
        self.imp().back_button.get()
    }

    pub fn search_entry(&self) -> gtk::SearchEntry {
        self.imp().search_entry.get()
    }

    pub fn search_results(&self) -> gtk::FlowBox {
        self.imp().search_results.get()
    }

    pub fn search_result_at_index(&self, index: &i32) -> Option<Podcast> {
        if let Some(ref model) = *self.imp().model.borrow() {
            let Some(object) = model
                .item(index.clone().try_into().expect("Failed to cast index"))
            else {
                return None;
            };

            if let Ok(data) = object.downcast::<Podcast>() {
                Some(data)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn load_search_results(&self, query: &str) {}
}
