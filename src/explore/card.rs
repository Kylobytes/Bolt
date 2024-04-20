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

use std::{
    cell::{Cell, RefCell},
    path::PathBuf,
    sync::OnceLock,
};

use gtk::{
    gdk,
    gdk_pixbuf::Pixbuf,
    gio,
    glib::{self, clone, closure_local, subclass::Signal},
    prelude::*,
    subclass::prelude::*,
};

use crate::{
    api::search::result::SearchResult, data::podcast, runtime, storage,
};

mod imp {
    use super::*;

    #[derive(gtk::CompositeTemplate, Debug, Default)]
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
        pub podcast_id: Cell<i64>,
        pub image_url: RefCell<Option<String>>,
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

    impl ObjectImpl for ExploreCard {
        fn constructed(&self) {
            self.parent_constructed();

            self.obj().connect_signals();
        }

        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();

            SIGNALS.get_or_init(|| {
                vec![Signal::builder("image-found")
                    .param_types([bool::static_type()])
                    .build()]
            })
        }
    }

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

    pub fn unsubscribe_button(&self) -> gtk::Button {
        self.imp().unsubscribe_button.get()
    }

    pub fn set_name(&self, name: &str) {
        self.imp().name.set_text(name);
    }

    pub fn set_description(&self, content: &str) {
        self.imp().description.set_text(content);
    }

    pub fn load_image(&self) {
        let imp = self.imp();
        let id: i64 = imp.podcast_id.get();

        imp.picture.get().set_visible(false);
        imp.image_missing_icon.get().set_visible(false);
        imp.picture_spinner.get().set_visible(true);

        let image_path: Option<PathBuf> =
            storage::podcast_image(&id.to_string());

        if image_path.is_none() && imp.image_url.borrow().is_none() {
            self.emit_by_name::<()>("image-found", &[&false]);

            return;
        }

        if image_path.is_some() {
            self.emit_by_name::<()>("image-found", &[&true]);

            return;
        }

        if let Some(ref image_url) = *imp.image_url.borrow() {
            let directory: PathBuf =
                storage::podcast_image_path(&id.to_string());
            let (sender, receiver) = async_channel::bounded::<bool>(1);

            runtime().spawn(
                clone!(@strong sender, @strong image_url => async move {
                    let result: Result<(), anyhow::Error> =
                    storage::save_image(&image_url, &directory).await;
                    let downloaded: bool = match result {
                        Ok(_) => true,
                        _ => false,
                    };

                    sender
                    .send(downloaded)
                    .await
                    .expect("The channel should be open");
                }),
            );

            glib::spawn_future_local(
                clone!(@strong receiver, @weak self as view => async move {
                    while let Ok(downloaded) = receiver.recv().await {
                        if downloaded {
                            view.emit_by_name::<()>("image-found", &[&true]);
                        } else {
                            view.emit_by_name::<()>("image-found", &[&false]);
                        }
                    }
                }),
            );
        }
    }

    pub fn subscribe(&self) {
        let imp = self.imp();
        let subscribe_button: gtk::Button = imp.subscribe_button.get();

        subscribe_button.set_sensitive(false);
        subscribe_button.set_label("Subscribing...");

        let (sender, receiver) = async_channel::bounded::<bool>(1);
        let id: i64 = self.imp().podcast_id.get();

        runtime().spawn(clone!(@strong sender, @strong id => async move {
            let subscribed = podcast::repository::subscribe(&id).await;

            sender.send(subscribed).await.expect("Channel must be open");
        }));

        glib::spawn_future_local(
            clone!(@strong receiver, @weak imp => async move {
                while let Ok(subscribed) = receiver.recv().await {
                    imp.subscribe_button.get().set_label("Subscribe");
                    imp.subscribe_button.get().set_sensitive(true);

                    if subscribed {
                        imp.subscribe_button.get().set_visible(false);
                        imp.unsubscribe_button.get().set_visible(true);
                    }
                }
            }),
        );
    }

    pub fn unsubscribe(&self) {
        self.imp()
            .unsubscribe_button
            .get()
            .set_label("Unsubscribing...");
        self.imp().unsubscribe_button.get().set_sensitive(false);

        let id: i64 = self.imp().podcast_id.get();

        let (sender, receiver) = async_channel::bounded::<bool>(1);

        runtime().spawn(clone!(@strong id, @strong sender => async move {
            let unsubscribed = podcast::repository::unsubscribe(&id).await;

            sender.send(unsubscribed).await.expect("The channel must be open");
        }));

        glib::spawn_future_local(
            clone!(@strong receiver, @weak self as view => async move {
                let subscribe_button = view.imp().subscribe_button.get();
                let unsubscribe_button = view.imp().unsubscribe_button.get();

                while let Ok(unsubscribed) = receiver.recv().await {

                    unsubscribe_button.set_label("Unsubscribe");
                    unsubscribe_button.set_sensitive(true);

                    if unsubscribed {
                        unsubscribe_button.set_visible(false);
                        subscribe_button.set_visible(true);
                    }
                }
            }),
        );
    }

    fn connect_signals(&self) {
        let imp = self.imp();

        self.connect_closure(
            "image-found",
            false,
            closure_local!(move |view: ExploreCard, found: bool| {
                if found {
                    let id: i64 = view.imp().podcast_id.get();
                    let image_path = storage::podcast_image(&id.to_string())
                        .expect("Image path must exist");
                    let pixbuf = Pixbuf::from_file_at_scale(
                        &image_path,
                        300,
                        300,
                        true,
                    );

                    if let Ok(pixbuf) = pixbuf {
                        let texture = gdk::Texture::for_pixbuf(&pixbuf);
                        view.imp().picture.get().set_paintable(Some(&texture));
                        view.imp().picture_spinner.get().set_visible(false);
                        view.imp().image_missing_icon.get().set_visible(false);
                        view.imp().picture.get().set_visible(true);
                    } else {
                        view.imp().picture_spinner.get().set_visible(false);
                        view.imp().picture.get().set_visible(false);
                        view.imp().image_missing_icon.get().set_visible(true);
                    }
                } else {
                    view.imp().picture_spinner.get().set_visible(false);
                    view.imp().picture.get().set_visible(false);
                    view.imp().image_missing_icon.get().set_visible(true);
                }
            }),
        );

        imp.subscribe_button.get().connect_clicked(
            clone!(@weak self as view => move |_| {
                view.subscribe();
            }),
        );

        imp.unsubscribe_button.get().connect_clicked(
            clone!(@weak self as view => move |_| {
                view.unsubscribe();
            }),
        );
    }
}

impl From<SearchResult> for ExploreCard {
    fn from(search_result: SearchResult) -> Self {
        let card = Self::default();
        let imp = card.imp();

        imp.name.set_text(&search_result.title);
        imp.description.set_text(&search_result.description);
        imp.podcast_id.set(search_result.id);

        if !search_result.image.is_empty() {
            imp.image_url.replace(Some(search_result.image));
        }

        card.load_image();

        card
    }
}
