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
 *https://api.podcastindex.org/api/1.0/recent/feeds?pretty
 * You should have received a copy of the GNU General Public License
 * along with Bolt. If not, see <https://www.gnu.org/licenses/>.
 *
 */

use std::cell::RefCell;

use gtk::{
    gio,
    glib::{self, clone},
    prelude::*,
    subclass::prelude::*,
};

use crate::{
    data::episode::object::EpisodeObject,
    episodes::{repository, row::EpisodeRow},
};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/kylobytes/Bolt/gtk/episodes/view.ui")]
    pub struct EpisodesView {
        #[template_child]
        pub episodes: TemplateChild<gtk::ListBox>,
        pub model: RefCell<Option<gio::ListStore>>,
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

    impl ObjectImpl for EpisodesView {
        fn constructed(&self) {
            self.parent_constructed();
            self.model
                .replace(Some(gio::ListStore::new::<EpisodeObject>()));

            let model_binding = self.model.borrow();
            let model = model_binding.as_ref();

            self.episodes.bind_model(model, move |item: &glib::Object| {
                let episode = item
                    .downcast_ref::<EpisodeObject>()
                    .expect("Item must be an episode");

                EpisodeRow::from(episode.to_owned()).into()
            });

            self.obj().load_episodes();
        }
    }

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

    pub fn load_episodes(&self) {
        glib::spawn_future_local(clone!(@weak self as view => async move {
            let episodes: Vec<EpisodeObject> = gio::spawn_blocking(|| { repository::load_episodes() })
                .await
                .expect("Failed to execute load episodes task")
                .into_iter()
                .map(EpisodeObject::from)
                .collect();

            let model_binding = view.imp().model.borrow();

            if let Some(model) = model_binding.as_ref() {
                model.extend_from_slice(&episodes);
            }
        }));
    }
}
