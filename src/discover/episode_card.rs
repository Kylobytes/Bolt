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
use std::cell::Cell;
use ureq::AgentBuilder;

use crate::{config::GETTEXT_PACKAGE, data::model::episode::Episode};

use super::discover_episode::DiscoverEpisode;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/kylobytes/Bolt/gtk/episode-card.ui")]
    pub struct EpisodeCard {
        #[template_child]
        pub image_spinner: TemplateChild<gtk::Spinner>,
        #[template_child]
        pub image: TemplateChild<gtk::Picture>,
        #[template_child]
        pub icon: TemplateChild<gtk::Image>,
        #[template_child]
        pub title: TemplateChild<gtk::Label>,
        #[template_child]
        pub show: TemplateChild<gtk::Label>,
        #[template_child]
        pub timestamp: TemplateChild<gtk::Label>,
        pub episode: Cell<i64>,
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

        view.imp().title.get().set_text(episode.title.as_str());

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

impl From<DiscoverEpisode> for EpisodeCard {
    fn from(episode: DiscoverEpisode) -> Self {
        let card = Self::default();
        let imp = card.imp();

        if let Some(title) = episode.title() {
            imp.title.get().set_text(&title);
        }

        if let Some(show) = episode.show() {
            imp.show.get().set_text(&show);
        }

        if episode.date_published() >= 0 {
            let datetime =
                NaiveDateTime::from_timestamp_opt(episode.date_published(), 0);

            if let Some(timestamp) = datetime {
                imp.timestamp
                    .get()
                    .set_text(&timestamp.format("%b %e, %Y").to_string());
            }
        }

        imp.episode.set(episode.id());

        card
    }
}

impl EpisodeCard {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn load_image(&self) {
        let episode_id = self.imp().episode.get();

        let mut episode_image_path = glib::user_cache_dir();
        episode_image_path.push(GETTEXT_PACKAGE);
        episode_image_path.push("images");
        episode_image_path.push("episodes");
        episode_image_path.push(episode_id.to_string());

        if episode_image_path.as_path().exists() {
            let image = gio::File::for_path(episode_image_path.as_path());
            self.imp().image.get().set_file(Some(&image));
            self.imp().image.get().set_visible(true);
            self.imp().image_spinner.get().stop();
            self.imp().image_spinner.get().set_visible(false);

            return;
        }

        let episode =
            gio::spawn_blocking(move || Episode::find_by_id(episode_id))
                .await
                .expect("Failed to acquire episode");

        if let Some(image_url) = episode.image_url {
            let image_path = episode_image_path.clone();

            gio::spawn_blocking(move || {
                let agent = AgentBuilder::new().build();
                let mut response = agent
                    .get(&image_url)
                    .call()
                    .expect("Failed to download image")
                    .into_reader();
                let mut image = std::fs::File::create(&image_path)
                    .expect("Failed to initialize image at path");
                std::io::copy(
                    &mut response,
                    &mut std::io::BufWriter::new(&mut image),
                )
                .expect("Failed to save image");
            })
            .await
            .expect("Failed to download image from url");

            let image = gio::File::for_path(episode_image_path.as_path());
            self.imp().image.get().set_file(Some(&image));
            self.imp().image.get().set_visible(true);
            self.imp().image_spinner.get().stop();
            self.imp().image_spinner.get().set_visible(false);

            return;
        }

        if let Some(show) = episode.show {
            let mut show_image_path = glib::user_cache_dir();
            show_image_path.push(GETTEXT_PACKAGE);
            show_image_path.push("images");
            show_image_path.push("shows");
            show_image_path.push(show.id.to_string());

            if show_image_path.as_path().exists() {
                let image = gio::File::for_path(show_image_path.as_path());
                self.imp().image.get().set_file(Some(&image));
                self.imp().image.get().set_visible(true);
                self.imp().image_spinner.get().stop();
                self.imp().image_spinner.get().set_visible(false);

                return;
            }

            if let Some(image_url) = show.image_url {
                let image_path = show_image_path.clone();

                gio::spawn_blocking(move || {
                    let agent = AgentBuilder::new().build();
                    let mut response = agent
                        .get(&image_url)
                        .call()
                        .expect("Could not download image")
                        .into_reader();
                    let mut image = std::fs::File::create(image_path)
                        .expect("Failed to initialize image file");
                    std::io::copy(
                        &mut response,
                        &mut std::io::BufWriter::new(&mut image),
                    )
                    .expect("Faild to save image");
                })
                .await
                .expect("Failed to download image from url");

                let image =
                    gio::File::for_path(show_image_path.clone().as_path());
                self.imp().image.get().set_file(Some(&image));
                self.imp().image.get().set_visible(true);
                self.imp().image_spinner.get().stop();
                self.imp().image_spinner.get().set_visible(false);

                return;
            }
        }

        self.imp().icon.get().set_visible(true);
        self.imp().image_spinner.get().stop();
        self.imp().image_spinner.get().set_visible(false);
    }
}
