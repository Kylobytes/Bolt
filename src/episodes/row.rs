/* row.rs
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

use chrono::DateTime;
use gtk::{
    gio,
    glib::{self, clone},
    prelude::WidgetExt,
    subclass::prelude::*,
};

use crate::{data::episode::object::EpisodeObject, utils};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/kylobytes/Bolt/gtk/episodes/row.ui")]
    pub struct EpisodeRow {
        #[template_child]
        pub picture: TemplateChild<gtk::Picture>,
        #[template_child]
        pub image_missing_icon: TemplateChild<gtk::Image>,
        #[template_child]
        pub title: TemplateChild<gtk::Label>,
        #[template_child]
        pub date: TemplateChild<gtk::Label>,
        pub image_url: RefCell<Option<String>>,
        pub episode_id: Cell<i64>,
        pub show_id: Cell<i64>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for EpisodeRow {
        const NAME: &'static str = "EpisodeRow";
        type Type = super::EpisodeRow;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for EpisodeRow {
        fn constructed(&self) {
            self.parent_constructed();

            self.obj().load_image();
        }
    }

    impl WidgetImpl for EpisodeRow {}
    impl BoxImpl for EpisodeRow {}
}

glib::wrapper! {
    pub struct EpisodeRow(ObjectSubclass<imp::EpisodeRow>)
        @extends gtk::Widget, gtk::Box,
    @implements gio::ActionGroup, gio::ActionMap;
}

impl Default for EpisodeRow {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl From<EpisodeObject> for EpisodeRow {
    fn from(episode: EpisodeObject) -> Self {
        let row = EpisodeRow::new();
        row.imp().image_url.replace(episode.image_url());
        row.imp().episode_id.replace(episode.id());
        row.imp().show_id.replace(episode.show_id());

        if let Some(title) = episode.title() {
            row.imp().title.get().set_label(&title);
        }

        if let Some(date) =
            DateTime::from_timestamp(episode.date_published(), 0)
        {
            let formatted_date = format!("{}", date.format("%b %d, %Y"));
            row.imp().date.get().set_label(&formatted_date);
        }

        row
    }
}

impl EpisodeRow {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_image(&self) {
        glib::spawn_future_local(clone!(@weak self as view => async move {
            let show_id = view.imp().show_id.get();
            let show_image_path = utils::show_image_path(&show_id.to_string());

            if show_image_path.as_path().exists() {
                let picture = view.imp().picture.get();
                let image = gio::File::for_path(show_image_path.as_path());

                picture.set_file(Some(&image));
                picture.set_visible(true);
            } else {
                view.imp().image_missing_icon.get().set_visible(true);
            }
            // }
        }));
    }
}
