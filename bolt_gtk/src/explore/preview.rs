/* preview.rs
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
 */

use std::cell::Cell;

use adw::{prelude::*, subclass::prelude::*};
use gtk::{
    gdk::Paintable,
    gio,
    glib::{self, clone},
};

use crate::{
    api::{episode::response::EpisodeResponse, episodes},
    data::podcast::{self, Podcast},
    runtime, storage,
};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/kylobytes/Bolt/gtk/explore/preview.ui")]
    pub struct Preview {
        #[template_child]
        pub back_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub picture: TemplateChild<gtk::Picture>,
        #[template_child]
        pub picture_container: TemplateChild<adw::Clamp>,
        #[template_child]
        pub picture_spinner: TemplateChild<gtk::Spinner>,
        #[template_child]
        pub spinner_container: TemplateChild<adw::Clamp>,
        #[template_child]
        pub image_missing_icon: TemplateChild<gtk::Image>,
        #[template_child]
        pub title: TemplateChild<gtk::Label>,
        #[template_child]
        pub description: TemplateChild<gtk::TextView>,
        #[template_child]
        pub subscribe_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub unsubscribe_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub episodes: TemplateChild<gtk::ListView>,
        pub podcast_id: Cell<Option<i64>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Preview {
        const NAME: &'static str = "Preview";
        type Type = super::Preview;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Preview {}
    impl WidgetImpl for Preview {}
    impl BinImpl for Preview {}
}

glib::wrapper! {
    pub struct Preview(ObjectSubclass<imp::Preview>)
        @extends gtk::Widget, adw::Bin,
    @implements gio::ActionGroup, gio::ActionMap;
}

impl Default for Preview {
    fn default() -> Self {
        glib::Object::new::<Self>()
    }
}

impl Preview {
    pub fn clear(&self) {
        let imp = self.imp();

        imp.description.get().buffer().set_text("");
        imp.picture
            .get()
            .set_paintable(Some(&Paintable::new_empty(0, 0)));
        imp.picture_container.get().set_visible(false);
        imp.spinner_container.get().set_visible(true);
        imp.podcast_id.set(None);

        let episodes = imp.episodes.get();

        if let Some(selection_model) = episodes.model() {
            let model = selection_model
                .downcast::<gtk::NoSelection>()
                .unwrap()
                .model()
                .unwrap()
                .downcast::<gtk::StringList>()
                .unwrap();

            while let Some(_row) = model.item(0) {
                model.remove(0);
            }
        }
    }

    pub fn back_button(&self) -> gtk::Button {
        self.imp().back_button.get()
    }

    pub fn subscribe_button(&self) -> gtk::Button {
        self.imp().subscribe_button.get()
    }

    pub fn unsubscribe_button(&self) -> gtk::Button {
        self.imp().unsubscribe_button.get()
    }

    pub fn load_podcast(&self, podcast: &Podcast) {
        let imp = self.imp();

        imp.title.get().set_label(&podcast.name());
        imp.podcast_id.set(Some(podcast.id().clone()));

        if let Some(description) = podcast.description() {
            let buffer = &imp.description.get().buffer();
            buffer.set_text(&description);
            imp.description.set_visible(true);
        } else {
            imp.description.set_visible(false);
        }

        imp.spinner_container.get().set_visible(false);

        if let Some(image_path) =
            storage::podcast_image(&podcast.id().to_string())
        {
            imp.picture.get().set_filename(Some(&image_path));
            imp.picture_container.get().set_visible(true);
            imp.image_missing_icon.get().set_visible(false);
        } else {
            imp.image_missing_icon.get().set_visible(true);
            imp.picture_container.get().set_visible(false);
        }

        let id = podcast.id().clone();
        self.load_episodes(&id);
        self.setup_subscribe_action(&id);
    }

    pub fn subscribe(&self) {
        let imp = self.imp();

        let subscribe_button = imp.subscribe_button.get();
        let (sender, receiver) = async_channel::bounded(1);

        subscribe_button.set_sensitive(false);
        subscribe_button.set_label("Subscribing...");

        let Some(id) = self.imp().podcast_id.get() else {
            return;
        };

        runtime().spawn(clone!(@strong id, @strong sender => async move {
            let subscribed = match podcast::repository::subscribe(&id).await {
                    Ok(_) => true,
                    _ => false
                };

            sender.send(subscribed).await.unwrap();
        }));

        glib::spawn_future_local(
            clone!(@weak self as view, @strong receiver => async move {
                let subscribe_button = view.imp().subscribe_button.get();
                let unsubscribe_button = view.imp().unsubscribe_button.get();

                while let Ok(subscribed) = receiver.recv().await {
                    subscribe_button.set_label("Subscribe");

                    if subscribed {
                        subscribe_button.set_sensitive(true);
                        subscribe_button.set_visible(false);
                        unsubscribe_button.set_visible(true);
                    }
                }
            }),
        );
    }

    pub fn unsubscribe(&self) {
        let imp = self.imp();

        let unsubscribe_button = imp.unsubscribe_button.get();
        let (sender, receiver) = async_channel::bounded(1);

        unsubscribe_button.set_sensitive(false);
        unsubscribe_button.set_label("Subscribing...");

        let Some(id) = self.imp().podcast_id.get() else {
            return;
        };

        runtime().spawn(clone!(@strong id, @strong sender => async move {
            podcast::repository::delete(&id).await;

            sender.send(true).await.unwrap();
        }));

        glib::spawn_future_local(
            clone!(@weak self as view, @strong receiver => async move {
                    let subscribe_button = view.imp().subscribe_button.get();
                    let unsubscribe_button = view.imp().unsubscribe_button.get();

                    while let Ok(_) = receiver.recv().await {
                        unsubscribe_button.set_label("Unsubscribe");
                        unsubscribe_button.set_sensitive(true);
                        unsubscribe_button.set_visible(false);
                        subscribe_button.set_visible(true);
                    }
            }),
        );
    }

    fn load_episodes(&self, id: &i64) {
        let (sender, receiver) = async_channel::bounded(1);

        runtime().spawn(clone!(@strong id, @strong sender => async move {
            let response: EpisodeResponse = episodes::by_feed_id(&id).await;
            sender.send(response.items).await.unwrap();
        }));

        glib::spawn_future_local(
            clone!(@strong receiver, @weak self as view => async move {
                while let Ok(episodes) = receiver.recv().await {
                    let titles: gtk::StringList = episodes
                        .iter()
                        .map(|episode| episode.title.clone()).collect();
                    let factory = gtk::SignalListItemFactory::new();

                    factory.connect_setup(|_, list_item| {
                        let title = gtk::Label::new(None);
                        title.set_halign(gtk::Align::Start);

                        let item = list_item
                            .downcast_ref::<gtk::ListItem>()
                            .expect("Item needs to be a ListItem");

                        item.set_child(Some(&title));
                        item.set_activatable(false);
                        item.property_expression("item")
                            .chain_property::<gtk::StringObject>("string")
                            .bind(&title, "label", gtk::Widget::NONE);
                    });

                    let selection_model = gtk::NoSelection::new(Some(titles));
                    view.imp().episodes.set_factory(Some(&factory));
                    view.imp().episodes.set_model(Some(&selection_model));
                }
            }),
        );
    }

    fn setup_subscribe_action(&self, id: &i64) {
        let (sender, receiver) = async_channel::bounded(1);

        runtime().spawn(clone!(@strong id, @strong sender => async move {
            let subscribed = podcast::repository::subscribed(&id).await;

            sender.send(subscribed).await.unwrap();
        }));

        glib::spawn_future_local(
            clone!(@weak self as view, @strong sender => async move {
                while let Ok(subscribed) = receiver.recv().await {
                    match subscribed {
                        true => view.imp().unsubscribe_button.get().set_visible(true),
                        false => view.imp().subscribe_button.get().set_visible(true)
                    }
                }
            }),
        );
    }
}
