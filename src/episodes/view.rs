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

use std::cell::{Cell, RefCell};

use gtk::{
    gio,
    glib::{self, clone, Cast},
    prelude::*,
    subclass::prelude::*,
};

use crate::{
    data::episode::{self, Episode},
    runtime,
};

use super::row::EpisodeRow;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/kylobytes/Bolt/gtk/episodes/view.ui")]
    pub struct EpisodesView {
        #[template_child]
        pub scrollbar: TemplateChild<gtk::ScrolledWindow>,
        #[template_child]
        pub episodes: TemplateChild<gtk::ListBox>,
        pub model: RefCell<Option<gio::ListStore>>,
        pub episode_count: Cell<u64>,
        pub current_offset: Cell<u64>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for EpisodesView {
        const NAME: &'static str = "EpisodesView";
        type Type = super::EpisodesView;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for EpisodesView {}
    impl WidgetImpl for EpisodesView {}
    impl BoxImpl for EpisodesView {}
}

glib::wrapper! {
    pub struct EpisodesView(ObjectSubclass<imp::EpisodesView>)
        @extends gtk::Widget, gtk::Box,
    @implements gio::ActionGroup, gio::ActionMap;
}

impl Default for EpisodesView {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl EpisodesView {
    pub fn new() -> Self {
        Self::default()
    }
}
