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
    gio,
    glib::{self, closure_local, subclass::Signal},
    prelude::*,
    subclass::prelude::*,
};

use crate::{api::search::result::SearchResult, storage};

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

            self.obj().load_image();
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
        let image: Option<PathBuf> = storage::podcast_image(&id.to_string());
        let ref image_url: Option<String> = *imp.image_url.borrow();

        if image.is_none() && image_url.is_none() {
            self.emit_by_name::<()>("image-found", &[&false]);

            return;
        }
    }

    pub fn subscribe(&self, id: &i64) {}
    pub fn unsubscribe(&self, id: &i64) {}

    fn connect_signals(&self) {
        let imp = self.imp();

        self.connect_closure(
            "image-found",
            false,
            closure_local!(move |view: ExploreCard, image_available: bool| {
                if !image_available {
                    view.imp().picture_spinner.get().set_visible(false);
                    view.imp().picture.get().set_visible(false);
                    view.imp().picture.get().set_visible(true);
                }
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

        card
    }
}
