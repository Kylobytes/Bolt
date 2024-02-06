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

use std::cell::Cell;

use gtk::{gdk, gdk_pixbuf, gio, glib, prelude::*, subclass::prelude::*};
use time::{macros::format_description, OffsetDateTime};

use crate::{data::episode::object::EpisodeObject, utils};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/kylobytes/Bolt/gtk/episodes/row.ui")]
    pub struct EpisodeRow {
        #[template_child]
        pub picture_container: TemplateChild<adw::Clamp>,
        #[template_child]
        pub picture: TemplateChild<gtk::Picture>,
        #[template_child]
        pub image_missing_icon: TemplateChild<gtk::Image>,
        #[template_child]
        pub title: TemplateChild<gtk::Label>,
        #[template_child]
        pub date: TemplateChild<gtk::Label>,
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

    impl ObjectImpl for EpisodeRow {}
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

        if let Some(title) = episode.title() {
            row.imp().title.get().set_label(&title);
        }

        let timestamp_format = format_description!("[unix_timestamp]");
        let datetime = OffsetDateTime::parse(
            &episode.date_published().to_string(),
            &timestamp_format,
        );

        if let Ok(date) = datetime {
            let date_format =
                format_description!("[month repr:short] [day], [year]");
            let formatted_date = date.format(&date_format).unwrap();
            row.imp().date.get().set_label(&formatted_date);
        }

        row.imp().episode_id.replace(episode.id());
        row.imp().show_id.replace(episode.show_id());
        row.load_image();

        row
    }
}

impl EpisodeRow {
    pub fn new() -> Self {
        Self::default()
    }

    fn load_image(&self) {
        let episode_id = self.imp().episode_id.get();
        let show_id = self.imp().show_id.get();

        let episode_image_path =
            utils::episode_image_path(&episode_id.to_string());
        let show_image_path = utils::show_image_path(&show_id.to_string());
        let episode_image_exists = episode_image_path.as_path().exists();
        let show_image_exists = show_image_path.as_path().exists();

        match (episode_image_exists, show_image_exists) {
            (true, _) => self.load_episode_image(&episode_id),
            (false, true) => self.load_show_image(&show_id),
            _ => {
                self.imp().picture_container.get().set_visible(false);
                self.imp().image_missing_icon.get().set_visible(true);
            }
        }
    }

    fn load_episode_image(&self, episode_id: &i64) {
        let image_path = utils::episode_image_path(&episode_id.to_string());

        let pixbuf = gdk_pixbuf::Pixbuf::from_file_at_scale(
            &image_path.as_path(),
            48,
            48,
            true,
        );

        if let Ok(pixbuf) = pixbuf {
            let texture = gdk::Texture::for_pixbuf(&pixbuf);

            self.imp().picture.get().set_paintable(Some(&texture));
            self.imp().picture_container.get().set_visible(true);
        } else {
            self.imp().picture_container.get().set_visible(false);
            self.imp().image_missing_icon.get().set_visible(true);
        }
    }

    fn load_show_image(&self, show_id: &i64) {
        let picture_container = self.imp().picture_container.get();
        let show_image_path = utils::show_image_path(&show_id.to_string());

        let pixbuf = gdk_pixbuf::Pixbuf::from_file_at_scale(
            &show_image_path.as_path(),
            48,
            48,
            true,
        );

        if let Ok(pixbuf) = pixbuf {
            let texture = gdk::Texture::for_pixbuf(&pixbuf);
            self.imp().picture.get().set_paintable(Some(&texture));
            picture_container.set_visible(true);
        } else {
            picture_container.set_visible(false);
            self.imp().image_missing_icon.get().set_visible(true);
        }
    }
}
