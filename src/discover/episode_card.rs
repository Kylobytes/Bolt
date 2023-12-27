/* episode_card.rs
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

use adw::prelude::*;
use chrono::NaiveDateTime;
use gtk::{gio, glib, subclass::prelude::*};

use crate::{config::GETTEXT_PACKAGE, data::model::episode::Episode};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/kylobytes/Bolt/gtk/episode-card.ui")]
    pub struct EpisodeCard {
        #[template_child]
        pub image: TemplateChild<gtk::Picture>,
        #[template_child]
        pub icon: TemplateChild<gtk::Image>,
        #[template_child]
        pub name: TemplateChild<gtk::Label>,
        #[template_child]
        pub show: TemplateChild<gtk::Label>,
        #[template_child]
        pub timestamp: TemplateChild<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for EpisodeCard {
        const NAME: &'static str = "EpisodeCard";
        type Type = super::EpisodeCard;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for EpisodeCard {}
    impl WidgetImpl for EpisodeCard {}
    impl BoxImpl for EpisodeCard {}
}

glib::wrapper! {
    pub struct EpisodeCard(ObjectSubclass<imp::EpisodeCard>)
        @extends gtk::Widget, gtk::Box,
    @implements gio::ActionGroup, gio::ActionMap;
}

impl Default for EpisodeCard {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl From<Episode> for EpisodeCard {
    fn from(episode: Episode) -> Self {
        let view = Self::default();

        view.imp().name.get().set_text(episode.title.as_str());

        let datetime =
            NaiveDateTime::from_timestamp_opt(episode.date_published, 0);

        if let Some(date) = datetime {
            view.imp()
                .timestamp
                .get()
                .set_text(&date.format("%b %e, %Y").to_string());
        }

        if let Some(show) = episode.show {
            view.imp().show.get().set_text(show.name.as_str());
        }

        view
    }
}

impl EpisodeCard {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn show_image(&self, feed_id: u64) {
        let mut image_path = glib::user_cache_dir();
        image_path.push(GETTEXT_PACKAGE);
        image_path.push("tmp");
        image_path.push("images");
        image_path.push(feed_id.to_string());

        if image_path.as_path().exists() {
            let image = gio::File::for_path(image_path.as_path());
            self.imp().image.get().set_file(Some(&image));
            self.imp().image.get().set_visible(true);
        } else {
            self.imp().icon.get().set_visible(true);
        };
    }
}
