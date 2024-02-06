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
 *
 * You should have received a copy of the GNU General Public License
 * along with Bolt. If not, see <https://www.gnu.org/licenses/>.
 *
 */

use std::cell::RefCell;

use adw::{prelude::*, subclass::prelude::*};
use gtk::{
    gdk, gdk_pixbuf,
    gio::{self, MemoryInputStream},
    glib::{self, clone},
};

use crate::{
    data::{episode::object::EpisodeObject, show::object::ShowObject},
    runtime,
    show_details::{self, episode_row::DiscoverEpisodeRow},
    utils,
};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/kylobytes/Bolt/gtk/show-details/view.ui")]
    pub struct ShowDetails {
        #[template_child]
        pub back_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub title: TemplateChild<gtk::Label>,
        #[template_child]
        pub picture_container: TemplateChild<adw::Clamp>,
        #[template_child]
        pub picture: TemplateChild<gtk::Picture>,
        #[template_child]
        pub spinner_container: TemplateChild<adw::Clamp>,
        #[template_child]
        pub picture_spinner: TemplateChild<gtk::Spinner>,
        #[template_child]
        pub image_missing_icon: TemplateChild<gtk::Image>,
        #[template_child]
        pub subscribe_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub description: TemplateChild<gtk::TextView>,
        #[template_child]
        pub podcasts: TemplateChild<gtk::ListBox>,
        pub model: RefCell<Option<gio::ListStore>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ShowDetails {
        const NAME: &'static str = "ShowDetails";
        type Type = super::ShowDetails;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ShowDetails {
        fn constructed(&self) {
            self.parent_constructed();

            self.model
                .replace(Some(gio::ListStore::new::<EpisodeObject>()));

            let model_binding = self.model.borrow();
            let model = model_binding.as_ref();

            self.podcasts.bind_model(model, move |item: &glib::Object| {
                let episode = item
                    .downcast_ref::<EpisodeObject>()
                    .expect("Item must be an episode");

                DiscoverEpisodeRow::from(episode.to_owned()).into()
            });
        }
    }
    impl WidgetImpl for ShowDetails {}
    impl BinImpl for ShowDetails {}
}

glib::wrapper! {
    pub struct ShowDetails(ObjectSubclass<imp::ShowDetails>)
        @extends gtk::Widget, adw::Bin,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl Default for ShowDetails {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl ShowDetails {
    pub fn back_button(&self) -> gtk::Button {
        self.imp().back_button.get()
    }

    pub fn load_details(&self, show: &ShowObject) {
        if let Some(title) = show.name() {
            self.imp().title.set_label(&title);
        }

        if let Some(description) = show.description() {
            let text_view = &self.imp().description.get();
            let buffer = text_view.buffer();
            let mut start = buffer.start_iter();
            let mut end = buffer.end_iter();
            buffer.delete(&mut start, &mut end);
            buffer.insert(&mut start, &description);
        }

        let show_id = show.id().clone();
        let (sender, receiver) = async_channel::bounded::<bool>(1);

        runtime().spawn(clone!(@strong show_id => async move {
            let subscribed = show_details::repository::check_subscribed(&show_id).await;
            sender.send(subscribed).await.expect("The channel should be open");
        }));

        glib::spawn_future_local(
            clone!(@weak self as view, @strong receiver => async move {
                if let Ok(subscribed) = receiver.recv().await {
                    if !subscribed {
                        view.imp().subscribe_button.get().set_visible(true);
                    }
                }
            }),
        );

        self.load_episodes(&show.id());
        self.load_image(&show.image_url());
    }

    pub fn load_episodes(&self, show_id: &i64) {
        glib::spawn_future_local(
            clone!(@weak self as view, @strong show_id => async move {
                let episodes: Vec<EpisodeObject> = gio::spawn_blocking(move || {
                    show_details::repository::load_show_episodes(show_id)
                }).await.expect("Failed to load episodes")
                    .into_iter()
                    .map(EpisodeObject::from)
                    .collect();

                if let Some(model) = view.imp().model.borrow().as_ref() {
                    model.remove_all();
                    model.extend_from_slice(&episodes);
                };
            }),
        );
    }

    pub fn load_image(&self, image_url: &Option<String>) {
        glib::spawn_future_local(
            clone!(@weak self as view, @strong image_url => async move {
                let image_missing_icon = view.imp().image_missing_icon.get();
                let spinner_container = view.imp().spinner_container.get();
                let picture_container = view.imp().picture_container.get();
                let picture_spinner = view.imp().picture_spinner.get();
                let picture = view.imp().picture.get();

                let Some(url) = image_url else {
                    picture_spinner.stop();
                    spinner_container.set_visible(false);
                    picture_container.set_visible(false);
                    image_missing_icon.set_visible(true);

                    return
                };

                if url.is_empty() {
                    picture_spinner.stop();
                    spinner_container.set_visible(false);
                    picture_container.set_visible(false);
                    image_missing_icon.set_visible(true);

                    return
                };

                let fetch_async_result = gio::spawn_blocking(move || {
                    utils::fetch_image(&url)
                }).await;

                let Ok(fetch_result) = fetch_async_result else {
                    picture_spinner.stop();
                    spinner_container.set_visible(false);
                    picture_container.set_visible(false);
                    image_missing_icon.set_visible(true);

                    return
                };

                if let Ok(content) = fetch_result {
                    let image_bytes = glib::Bytes::from(&content);
                    let stream = MemoryInputStream::from_bytes(&image_bytes);
                    let pixbuf = gdk_pixbuf::Pixbuf::from_stream_at_scale(
                        &stream,
                        328,
                        328,
                        true,
                        gio::Cancellable::NONE
                    ).unwrap();
                    let texture = gdk::Texture::for_pixbuf(&pixbuf);

                    picture.set_paintable(Some(&texture));
                    picture_spinner.stop();
                    spinner_container.set_visible(false);
                    picture_container.set_visible(true);
                } else {
                    picture_spinner.stop();
                    spinner_container.set_visible(false);
                    picture_container.set_visible(false);
                    image_missing_icon.set_visible(true);
                }
            }),
        );
    }
}
