/* discover_view.rs
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
 *https://api.podcastindex.org/api/1.0/recent/feeds?pretty
 * You should have received a copy of the GNU General Public License
 * along with Bolt. If not, see <https://www.gnu.org/licenses/>.
 *
 */

use std::cell::RefCell;

use adw::prelude::*;
use gtk::glib::{self, subclass::prelude::*, Properties};

use crate::data::model::episode::Episode;

#[derive(Clone, Debug)]
pub struct EpisodeData {
    pub id: i64,
    pub title: Option<String>,
    pub date_published: i64,
    pub show: Option<String>,
    pub show_id: i64,
}

impl Default for EpisodeData {
    fn default() -> Self {
        EpisodeData {
            id: -1,
            title: None,
            date_published: -1,
            show: None,
            show_id: -1,
        }
    }
}

mod imp {
    use super::*;

    #[derive(Debug, Default, Properties)]
    #[properties(wrapper_type = super::DiscoverEpisode)]
    pub struct DiscoverEpisode {
        #[property(name = "id", get, construct_only, type = i64, member = id)]
        #[property(name = "title", get, construct_only, type = Option<String>, member = title)]
        #[property(name = "date-published", get, construct_only, type = i64, member = date_published)]
        #[property(name = "show", get, construct_only, type = Option<String>, member = show)]
        #[property(name = "show-id", get, construct_only, type = i64, member = show_id)]
        data: RefCell<EpisodeData>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DiscoverEpisode {
        const NAME: &'static str = "DiscoverEpisode";
        type Type = super::DiscoverEpisode;
    }

    #[glib::derived_properties]
    impl ObjectImpl for DiscoverEpisode {}
}

glib::wrapper! {
    pub struct DiscoverEpisode(ObjectSubclass<imp::DiscoverEpisode>);
}

impl Default for DiscoverEpisode {
    fn default() -> Self {
        glib::Object::builder::<Self>().build()
    }
}

impl DiscoverEpisode {
    pub fn new(episode: Episode) -> Self {
        let mut show_name: Option<String> = None;
        let mut show_id: i64 = -1;

        if let Some(show) = episode.show {
            show_name = Some(show.name);
            show_id = show.id;
        }

        glib::Object::builder::<Self>()
            .property("id", episode.id)
            .property("title", Some(episode.title))
            .property("date-published", episode.date_published)
            .property("show", show_name)
            .property("show-id", show_id)
            .build()
    }
}
