/* episode_row.rs
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

use chrono::DateTime;
use gtk::{gio, glib, subclass::prelude::*};

use crate::data::episode::object::EpisodeObject;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(
        resource = "/com/kylobytes/Bolt/gtk/show-details/episode-row.ui"
    )]
    pub struct DiscoverEpisodeRow {
        #[template_child]
        pub episode_title: TemplateChild<gtk::Label>,
        #[template_child]
        pub episode_date: TemplateChild<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DiscoverEpisodeRow {
        const NAME: &'static str = "DiscoverEpisodeRow";
        type Type = super::DiscoverEpisodeRow;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for DiscoverEpisodeRow {}
    impl WidgetImpl for DiscoverEpisodeRow {}
    impl BoxImpl for DiscoverEpisodeRow {}
}

glib::wrapper! {
    pub struct DiscoverEpisodeRow(ObjectSubclass<imp::DiscoverEpisodeRow>)
        @extends gtk::Widget, gtk::Box,
    @implements gio::ActionGroup, gio::ActionMap;
}

impl Default for DiscoverEpisodeRow {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl From<EpisodeObject> for DiscoverEpisodeRow {
    fn from(episode: EpisodeObject) -> Self {
        let row = DiscoverEpisodeRow::new();

        if let Some(title) = episode.title() {
            row.imp().episode_title.get().set_label(&title);
        };

        if let Some(date) =
            DateTime::from_timestamp(episode.date_published(), 0)
        {
            let formatted_date = format!("{}", date.format("%b %d, %Y"));
            row.imp().episode_date.get().set_label(&formatted_date);
        };

        row
    }
}

impl DiscoverEpisodeRow {
    pub fn new() -> Self {
        Self::default()
    }
}
