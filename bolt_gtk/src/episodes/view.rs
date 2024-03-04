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

use std::cell::{Cell, RefCell};

use gtk::{
    gio::{self, ListStore},
    glib::{self, clone, closure_local},
    prelude::*,
    subclass::prelude::*,
};

use bolt_core::{database, episode};

use crate::{
    data::episode::object::EpisodeObject,
    episodes::{repository, row::EpisodeRow},
    runtime, storage, utils,
};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/kylobytes/Bolt/gtk/episodes/view.ui")]
    pub struct EpisodesView {
        #[template_child]
        pub scrollbar: TemplateChild<gtk::ScrolledWindow>,
        #[template_child]
        pub episodes: TemplateChild<gtk::ListBox>,
        pub model: RefCell<Option<ListStore>>,
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

    pub fn scrollbar(&self) -> gtk::ScrolledWindow {
        self.imp().scrollbar.get()
    }

    pub fn setup_model(&self, model: &ListStore) {
        self.imp().model.replace(Some(model.clone()));
        self.imp().episodes.get().bind_model(
            Some(model),
            move |item: &glib::Object| {
                let episode = item
                    .downcast_ref::<EpisodeObject>()
                    .expect("Item must be an episode");

                let row = EpisodeRow::from(episode.to_owned());
                let media_url = &episode.media_url();
                let id = &episode.id();
                let show_id = &episode.show_id();

                row.connect_closure(
                    "download-triggered",
                    false,
                    closure_local!(
                        @strong media_url,
                        @strong id,
                        @strong show_id
                            => move |_row: EpisodeRow| {
                                let directory = storage::episode_path(&id.to_string(), &show_id.to_string());
                                runtime()
                                    .spawn(
                                        clone!(
                                            @strong media_url,
                                            @strong directory,
                                            @strong id => async move {
                                                utils::download_episode_media(
                                                    &media_url,
                                                    &directory
                                                ).await;

                                                repository::queue(&id).await;
                                            }));
                            }),
                );

                row.into()
            },
        );
    }

    pub fn reload_episodes(&self) {
        if let Some(ref model) = *self.imp().model.borrow() {
            self.imp().current_offset.set(0);
            model.remove_all();
            self.load_episodes();
        }
    }

    pub fn load_episodes(&self) {
        let offset = self.imp().current_offset.get();
        let (sender, receiver) = async_channel::bounded(1);

        runtime().spawn(clone!(@strong sender, @strong offset => async move {
            let database_url = utils::database_url();
            let connection = database::connect(&database_url).await;
            let episodes = episode::repository::load_episodes(&connection, &offset).await;

            sender.send(episodes).await.expect("The channel needs to be open");
        }));

        glib::spawn_future_local(
            clone!(@weak self as view, @strong receiver => async move {
                while let Ok(episodes) = receiver.recv().await {
                    let Some(ref model) = *view.imp().model.borrow() else {
                        return;
                    };

                    let new_offset = u64::try_from(episodes.len()).unwrap() + view.imp().current_offset.get();
                    view.imp().current_offset.set(new_offset.into());

                    let episode_objects: Vec<EpisodeObject> = episodes
                        .into_iter()
                        .map(|episode| {
                            EpisodeObject::from(episode.0)
                        })
                        .collect();

                    model.extend_from_slice(&episode_objects);
                }
            }),
        );
    }

    pub fn load_episode_count(&self) {
        let (sender, receiver) = async_channel::bounded(1);

        runtime().spawn(clone!(@strong sender => async move {
            let database_url = utils::database_url();
            let connection = database::connect(&database_url).await;
            let episode_count = episode::repository::load_episode_count(connection).await;
            sender.send(episode_count).await.expect("The channel needs to be open");
        }));

        glib::spawn_future_local(
            clone!(@weak self as view, @strong receiver => async move {
                if let Ok(count) = receiver.recv().await {
                    view.imp().episode_count.set(count);
                }
            }),
        );
    }
}
