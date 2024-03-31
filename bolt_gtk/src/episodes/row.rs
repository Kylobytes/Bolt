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

use std::{cell::Cell, sync::OnceLock};

use gtk::{
    gdk, gdk_pixbuf, gio,
    glib::{self, clone, subclass::Signal},
    prelude::*,
    subclass::prelude::*,
};
use time::{macros::format_description, OffsetDateTime};

use crate::{data::episode::Episode, storage};

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
        #[template_child]
        pub download_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub progress_bar: TemplateChild<gtk::ProgressBar>,
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
        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![Signal::builder("download-triggered").build()]
            })
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

impl From<Episode> for EpisodeRow {
    fn from(episode: Episode) -> Self {
        let row = EpisodeRow::new();
        row.imp().title.get().set_label(&episode.title());

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
        row.imp().show_id.replace(episode.podcast_id());
        row.load_image();

        row
    }
}

impl EpisodeRow {
    pub fn new() -> Self {
        let row = Self::default();
        row.connect_signals();

        row
    }

    fn connect_signals(&self) {
        self.imp().download_button.get().connect_clicked(
            clone!(@weak self as row => move |button| {
                button.set_sensitive(false);
                row.emit_by_name::<()>("download-triggered", &[]);
            }),
        );
    }

    fn load_image(&self) {
        let episode_id = self.imp().episode_id.get();
        let show_id = self.imp().show_id.get();

        let episode_image_path = storage::episode_path(
            &episode_id.to_string(),
            &show_id.to_string(),
        );

        let show_image_path = storage::podcast_path(&show_id.to_string());
        let episode_image_exists = episode_image_path.as_path().exists()
            && std::fs::read_dir(&episode_image_path)
                .expect(
                    "Failed to read the contents of the episode's directory",
                )
                .filter_map(|file| file.ok())
                .filter(|file| file.path().is_file())
                .filter(|file| {
                    let Ok(filename) = file.file_name().into_string() else {
                        return false;
                    };

                    return filename == "cover.png" || filename == "cover.jpg";
                })
                .count()
                > 0;

        let show_image_exists = show_image_path.as_path().exists()
            && std::fs::read_dir(&show_image_path)
                .expect("Failed to read the contents of the show's directory")
                .filter_map(|file| file.ok())
                .filter(|file| file.path().is_file())
                .filter(|file| {
                    let Ok(filename) = file.file_name().into_string() else {
                        return false;
                    };

                    return filename == "cover.png" || filename == "cover.jpg";
                })
                .count()
                > 0;

        match (episode_image_exists, show_image_exists) {
            (true, _) => self.load_episode_image(&episode_id, &show_id),
            (false, true) => self.load_show_image(&show_id),
            _ => {
                self.imp().picture_container.get().set_visible(false);
                self.imp().image_missing_icon.get().set_visible(true);
            }
        }
    }

    fn load_episode_image(&self, episode_id: &i64, show_id: &i64) {
        let Some(image_path) = storage::episode_image(
            &episode_id.to_string(),
            &show_id.to_string(),
        ) else {
            return;
        };

        let pixbuf =
            gdk_pixbuf::Pixbuf::from_file_at_scale(&image_path, 48, 48, true);

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
        let Some(show_image_path) =
            storage::podcast_image(&show_id.to_string())
        else {
            return;
        };

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
