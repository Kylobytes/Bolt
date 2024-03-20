/* card.rs
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
 */

use gtk::{
    gio,
    glib::{self, clone},
    prelude::*,
    subclass::prelude::*,
};

use crate::{data::podcast, runtime, storage};

mod imp {

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/kylobytes/Bolt/gtk/explore/card.ui")]
    pub struct ExploreCard {
        #[template_child]
        pub picture_spinner: TemplateChild<gtk::Spinner>,
        #[template_child]
        pub picture: TemplateChild<gtk::Picture>,
        #[template_child]
        pub image_missing_icon: TemplateChild<gtk::Image>,
        #[template_child]
        pub name: TemplateChild<gtk::Label>,
        #[template_child]
        pub description: TemplateChild<gtk::Label>,
        #[template_child]
        pub unsubscribe_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub subscribe_button: TemplateChild<gtk::Button>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ExploreCard {
        const NAME: &'static str = "ExploreCard";
        type Type = super::ExploreCard;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ExploreCard {}
    impl WidgetImpl for ExploreCard {}
    impl BoxImpl for ExploreCard {}
}

glib::wrapper! {
    pub struct ExploreCard(ObjectSubclass<imp::ExploreCard>)
        @extends gtk::Widget, gtk::Box,
    @implements gio::ActionGroup, gio::ActionMap;
}

impl Default for ExploreCard {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl ExploreCard {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn subscribe_button(&self) -> gtk::Button {
        self.imp().subscribe_button.get()
    }

    pub fn set_name(&self, name: &str) {
        self.imp().name.set_text(name);
    }

    pub fn set_description(&self, content: &str) {
        self.imp().description.set_text(content);
    }

    pub fn load_image(&self, id: &i64, image_url: &str) {
        self.imp().picture.get().set_visible(false);
        self.imp().image_missing_icon.get().set_visible(false);
        self.imp().picture_spinner.get().set_visible(true);

        if let Some(image) = storage::podcast_image(&id.to_string()) {
            self.imp().picture.get().set_filename(Some(&image));
            self.imp().picture.get().set_visible(true);
            self.imp().image_missing_icon.get().set_visible(false);
            self.imp().picture_spinner.get().set_visible(false);

            return;
        }

        let image_path = storage::podcast_image_cache(&id.to_string());
        let image_link = image_url.to_string();
        let (sender, receiver) = async_channel::bounded(1);

        runtime().spawn(clone!(
            @strong sender,
            @strong image_path,
            @strong image_link as image_url,
            @strong id => async move {
                let result = storage::save_image(
                    &image_url,
                    &image_path,
                    "cover"
                ).await;

                let saved = match result {
                    Ok(_) => true,
                    _ => false
                };

                sender.send(saved).await.unwrap()
        }));

        glib::spawn_future_local(
            clone!(@weak self as view, @strong id => async move {
                let picture_spinner = view.imp().picture_spinner.get();
                let picture = view.imp().picture.get();
                let image_missing_icon = view.imp().image_missing_icon.get();

                while let Ok(saved) = receiver.recv().await {
                    picture_spinner.set_visible(false);

                    if saved {
                        let Some(image) = storage::podcast_image(
                            &id.to_string()
                        ) else {
                            image_missing_icon.set_visible(true);
                            picture.set_visible(false);

                            return;
                        };

                        picture.set_filename(Some(&image));
                        picture.set_visible(true);
                        image_missing_icon.set_visible(false);
                    } else {
                        image_missing_icon.set_visible(true);
                        picture.set_visible(false);
                    }
                }
            }),
        );
    }

    pub fn subscribe(&self, id: &i64) {
        let subscribe_button = self.subscribe_button();

        subscribe_button.set_sensitive(false);
        subscribe_button.set_label("Subscribing...");

        let (sender, receiver) = async_channel::bounded::<bool>(1);

        runtime().spawn(clone!(@strong id, @strong sender => async move {
            let response = podcast::repository::subscribe(&id).await;

            let saved = match response {
                Ok(_) => true,
                _ => false
            };

            sender.send(saved).await.unwrap()
        }));

        let unsubscribe_button = self.imp().unsubscribe_button.get();

        glib::spawn_future_local(
            clone!(@weak subscribe_button, @weak unsubscribe_button, @strong receiver => async move {
                while let Ok(saved) = receiver.recv().await {
                    if saved {
                        subscribe_button.set_visible(false);
                        unsubscribe_button.set_visible(true);
                    } else {
                        subscribe_button.set_label("Subscribe");
                        subscribe_button.set_sensitive(true);
                    }
                }
            }),
        );
    }
}
