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
 *
 */

use adw::prelude::*;
use gtk::{
    gdk, gdk_pixbuf,
    gio::{self, MemoryInputStream},
    glib::{self, clone},
    subclass::prelude::*,
};
use std::cell::{Cell, RefCell};

use crate::{data::show::object::ShowObject, discover, runtime, utils};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/kylobytes/Bolt/gtk/discover/card.ui")]
    pub struct DiscoverCard {
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
        pub subscribe_button: TemplateChild<gtk::Button>,
        pub show_id: Cell<i64>,
        pub image_url: RefCell<Option<String>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DiscoverCard {
        const NAME: &'static str = "DiscoverCard";
        type Type = super::DiscoverCard;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for DiscoverCard {}
    impl WidgetImpl for DiscoverCard {}
    impl BoxImpl for DiscoverCard {}
}

glib::wrapper! {
    pub struct DiscoverCard(ObjectSubclass<imp::DiscoverCard>)
        @extends gtk::Widget, gtk::Box,
    @implements gio::ActionGroup, gio::ActionMap;
}

impl Default for DiscoverCard {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl From<ShowObject> for DiscoverCard {
    fn from(show: ShowObject) -> Self {
        let card = Self::default();
        let imp = card.imp();

        if let Some(name) = show.name() {
            imp.name.get().set_text(&name);
        }

        if let Some(description) = show.description() {
            imp.description.get().set_text(&description);
        }

        if show.subscribed() {
            card.mark_subscribed();
        }

        imp.show_id.set(show.id());
        card.connect_signals();

        if let Some(image_url) = show.image_url() {
            if image_url.is_empty() {
                imp.picture_spinner.get().stop();
                imp.picture_spinner.get().set_visible(false);
                imp.image_missing_icon.get().set_visible(true);
            } else {
                let (sender, receiver) = async_channel::bounded(1);

                runtime().spawn(clone!(@strong image_url => async move {
                    let image = utils::fetch_image(&image_url).await;
                    sender.send(image).await.expect("The image channel should be open");
                }));

                glib::spawn_future_local(clone!(@weak card => async move {
                    if let Ok(result) = receiver.recv().await {
                        let picture_spinner = card.imp().picture_spinner.get();

                        if let Ok(content) = result {
                            let image_bytes = glib::Bytes::from(&content);
                            let stream = MemoryInputStream::from_bytes(&image_bytes);
                            let pixbuf = gdk_pixbuf::Pixbuf::from_stream_at_scale(
                                &stream,
                                328,
                                328,
                                true,
                                gio::Cancellable::NONE
                            );

                            if let Ok(pixbuf) = pixbuf {
                                let texture = gdk::Texture::for_pixbuf(&pixbuf);
                                let picture = card.imp().picture.get();

                                picture.set_paintable(Some(&texture));
                                picture.set_visible(true);
                            } else {
                                card.imp().image_missing_icon.get().set_visible(true);
                            }
                        } else {
                            card.imp().image_missing_icon.get().set_visible(true);
                        }

                        picture_spinner.stop();
                        picture_spinner.set_visible(false);
                    }
                }));
            }
        }

        card
    }
}

impl DiscoverCard {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn connect_signals(&self) {
        let (sender, receiver) = async_channel::bounded::<bool>(1);
        let button = self.imp().subscribe_button.get();

        button.connect_clicked(
            clone!(@weak self as view, @strong sender => move |button: &gtk::Button| {
                button.set_label("Subscribing...");
                button.set_sensitive(false);
                let show_id = view.imp().show_id.get();

                runtime().spawn(clone!(@strong show_id, @strong sender => async move {
                    discover::repository::subscribe(&show_id).await;
                    sender.send(true).await.expect("The channel should be open");
                }));
            }),
        );

        glib::spawn_future_local(
            clone!(@weak button, @strong receiver => async move {
                if let Ok(save_successful) = receiver.recv().await {
                    if save_successful {
                        button.set_label("Subscribed");
                    }
                }
            }),
        );
    }

    pub fn mark_subscribed(&self) {
        let subscribe_button: gtk::Button = self.imp().subscribe_button.get();

        subscribe_button.set_sensitive(false);
        subscribe_button.set_label("Subscribed");
    }
}
