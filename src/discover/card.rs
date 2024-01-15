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
    gio,
    glib::{self, clone},
    subclass::prelude::*,
};
use std::cell::{Cell, RefCell};

use crate::{
    discover::{self, show::DiscoverShow},
    utils::show_image_path,
};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/kylobytes/Bolt/gtk/discover/card.ui")]
    pub struct DiscoverCard {
        #[template_child]
        pub image_spinner: TemplateChild<gtk::Spinner>,
        #[template_child]
        pub image: TemplateChild<gtk::Picture>,
        #[template_child]
        pub icon: TemplateChild<gtk::Image>,
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

    impl ObjectImpl for DiscoverCard {
        fn constructed(&self) {
            self.parent_constructed();

            self.obj().load_image();
            self.obj().connect_signals();
        }
    }
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

impl From<DiscoverShow> for DiscoverCard {
    fn from(show: DiscoverShow) -> Self {
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
        imp.image_url.replace(show.image_url());

        card
    }
}

impl DiscoverCard {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_image(&self) {
        glib::spawn_future_local(clone!(@weak self as view => async move {
            let show_id = view.imp().show_id.get();
            let image_url = view.imp().image_url.take();

            let image_path = show_image_path(&show_id.to_string());

            if image_path.as_path().exists() {
                let image = gio::File::for_path(&image_path.as_path());
                view.imp().image.get().set_file(Some(&image));
                view.imp().image_spinner.get().stop();
                view.imp().image_spinner.get().set_visible(false);
                view.imp().image.get().set_visible(true);

                return;
            }

            let Some(url) = image_url else {
                view.imp().image_spinner.get().stop();
                view.imp().image_spinner.get().set_visible(false);
                view.imp().icon.get().set_visible(true);

                return;
            };

            let destination = image_path.clone();

            let image_saved = gio::spawn_blocking(move || {
                discover::repository::save_image(&url, &destination)
            })
                .await;

            if let Ok(_) = image_saved {
                let image = gio::File::for_path(image_path.as_path());
                view.imp().image.get().set_file(Some(&image));
                view.imp().image.get().set_visible(true);
            } else {
                view.imp().icon.get().set_visible(true);
            }

            view.imp().image_spinner.get().stop();
            view.imp().image_spinner.get().set_visible(false);
        }));
    }

    pub fn connect_signals(&self) {
        self.imp().subscribe_button.get().connect_clicked(
            clone!(@weak self as view => move |button: &gtk::Button| {
                button.set_label("Subscribing...");
                button.set_sensitive(false);
                let show_id = view.imp().show_id.get();

                glib::spawn_future_local(
                    clone!(@weak button, @strong show_id => async move {
                        gio::spawn_blocking(move || {
                            discover::repository::subscribe(show_id);
                        }).await.expect("Failed to finish subscribe task");

                        button.set_label("Subscribed");
                    })
                );
            }),
        );
    }

    pub fn mark_subscribed(&self) {
        let subscribe_button: gtk::Button = self.imp().subscribe_button.get();

        subscribe_button.set_sensitive(false);
        subscribe_button.set_label("Subscribed");
    }
}
