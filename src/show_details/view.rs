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
    gio,
    glib::{self, clone},
};

use crate::{
    data::{episode::object::EpisodeObject, show::object::ShowObject},
    show_details,
    utils::{self, show_image_path},
};

mod imp {
    use crate::show_details::episode_row::EpisodeRow;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/kylobytes/Bolt/gtk/show-details/view.ui")]
    pub struct ShowDetails {
        #[template_child]
        pub title: TemplateChild<gtk::Label>,
        #[template_child]
        pub image: TemplateChild<gtk::Picture>,
        #[template_child]
        pub image_spinner: TemplateChild<gtk::Spinner>,
        #[template_child]
        pub noimage: TemplateChild<gtk::Image>,
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

                EpisodeRow::from(episode.to_owned()).into()
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

        glib::spawn_future_local(
            clone!(@weak self as view, @strong show_id => async move {
                let show_exists = gio::spawn_blocking(move || {
                    show_details::repository::check_show_exists(&show_id)}
                ).await.expect("Failed to check whether show exists");

                if !show_exists {
                    view.imp().subscribe_button.get().set_visible(true);
                }
            }),
        );

        self.load_episodes(&show.id());
        self.load_image(show.id(), &show.image_url());
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

    pub fn load_image(&self, show_id: i64, image_url: &Option<String>) {
        glib::spawn_future_local(
            clone!(@weak self as view, @strong show_id, @strong image_url => async move {
                let image_path = show_image_path(&show_id.to_string());

                view.imp().image_spinner.get().set_visible(true);

                if image_path.as_path().exists() {
                    view.imp().image_spinner.get().stop();
                    view.imp().image_spinner.get().set_visible(false);

                    let image = gio::File::for_path(&image_path.as_path());
                    view.imp().image.get().set_file(Some(&image));
                    view.imp().image.get().set_visible(true);

                    return;
                }

                let Some(url) = image_url else {
                    view.imp().image_spinner.get().stop();
                    view.imp().image_spinner.get().set_visible(false);
                    view.imp().noimage.get().set_visible(true);

                    return;
                };

                let destination = image_path.clone();

                let image_saved = gio::spawn_blocking(move || {
                    utils::save_image(&url, &destination)
                }).await;

                if let Ok(_) = image_saved {
                    let image = gio::File::for_path(image_path.as_path());
                    view.imp().image.get().set_file(Some(&image));
                    view.imp().image.get().set_visible(true);
                } else {
                    view.imp().noimage.get().set_visible(true);
                }

                view.imp().image_spinner.get().stop();
                view.imp().image_spinner.get().set_visible(false);
            }),
        );
    }
}
