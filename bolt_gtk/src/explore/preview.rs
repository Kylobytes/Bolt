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

use adw::{prelude::*, subclass::prelude::*};
use gtk::{
    gio,
    glib::{self, clone},
};

use crate::{
    api::{episode::response::EpisodeResponse, episodes},
    data::podcast::Podcast,
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
        pub episodes: TemplateChild<gtk::ListView>,
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
        imp.picture.get().set_pixbuf(None);
        imp.picture_container.get().set_visible(false);
        imp.spinner_container.get().set_visible(true);

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

    pub fn load_podcast(&self, podcast: &Podcast) {
        let imp = self.imp();

        imp.title.get().set_label(&podcast.name());

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
}
