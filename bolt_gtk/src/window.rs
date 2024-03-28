/* window.rs
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

use adw::subclass::prelude::*;
use gtk::{
    gio,
    glib::{self, clone},
    prelude::*,
};

use crate::{
    data::podcast, empty::view::EmptyView, episodes::view::EpisodesView,
    explore::view::ExploreView, queue_view::QueueView, runtime,
    show_details::view::ShowDetails,
};

#[derive(Clone, Debug)]
pub enum View {
    Loading,
    Empty,
    Explore,
    Podcasts,
    Preview,
    ShowDetails,
}

mod imp {
    use crate::explore::preview::Preview;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/kylobytes/Bolt/gtk/window.ui")]
    pub struct BoltWindow {
        #[template_child]
        pub main_stack: TemplateChild<gtk::Stack>,
        #[template_child]
        pub btn_explore: TemplateChild<gtk::Button>,
        #[template_child]
        pub btn_refresh: TemplateChild<gtk::Button>,
        #[template_child]
        pub explore_view: TemplateChild<ExploreView>,
        #[template_child]
        pub empty_view: TemplateChild<EmptyView>,
        #[template_child]
        pub queue_view: TemplateChild<QueueView>,
        #[template_child]
        pub episodes_view: TemplateChild<EpisodesView>,
        #[template_child]
        pub show_details_view: TemplateChild<ShowDetails>,
        #[template_child]
        pub preview: TemplateChild<Preview>,
        #[template_child]
        pub podcasts_stack: TemplateChild<adw::ViewStack>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for BoltWindow {
        const NAME: &'static str = "BoltWindow";
        type Type = super::BoltWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for BoltWindow {}
    impl WidgetImpl for BoltWindow {}
    impl WindowImpl for BoltWindow {}
    impl ApplicationWindowImpl for BoltWindow {}
    impl AdwApplicationWindowImpl for BoltWindow {}
}

glib::wrapper! {
    pub struct BoltWindow(ObjectSubclass<imp::BoltWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
    @implements gio::ActionGroup, gio::ActionMap;
}

impl BoltWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        let window = glib::Object::builder::<BoltWindow>()
            .property("application", application)
            .build();

        window.connect_signals();
        window.setup_explore();
        window.initialize();

        window
    }

    fn initialize(&self) {
        self.switch_view(View::Loading);

        let (sender, receiver) = async_channel::bounded(1);

        runtime().spawn(clone!(@strong sender => async move {
            let count = podcast::repository::load_count().await;

            sender.send(count).await.unwrap();
        }));

        glib::spawn_future_local(
            clone!(@weak self as window, @strong receiver => async move {
                while let Ok(count) = receiver.recv().await {
                    if count > 0 {
                        window.switch_view(View::Podcasts);
                    } else {
                        window.switch_view(View::Empty);
                    }
                }
            }),
        );
    }

    fn switch_view(&self, view: View) {
        let stack = self.imp().main_stack.get();

        match view {
            View::Loading => stack.set_visible_child_name("loading-view"),
            View::Empty => stack.set_visible_child_name("empty-view"),
            View::Explore => stack.set_visible_child_name("explore-view"),
            View::Podcasts => stack.set_visible_child_name("podcasts-view"),
            View::Preview => stack.set_visible_child_name("preview"),
            View::ShowDetails => {
                stack.set_visible_child_name("show-details-view")
            }
        }
    }

    fn connect_signals(&self) {
        self.imp().empty_view.get().btn_explore().connect_clicked(
            clone!(@weak self as window => move |_button| {
                window.switch_view(View::Explore);
            }),
        );

        self.imp().btn_explore.get().connect_clicked(
            clone!(@weak self as window => move |_button| {
                window.switch_view(View::Explore);
            }),
        );
    }

    fn setup_explore(&self) {
        let explore_view = self.imp().explore_view.get();
        let preview = self.imp().preview.get();

        explore_view.back_button().connect_clicked(
            clone!(@weak self as window => move |_button| {
                window.return_from_explore();
            }),
        );

        explore_view.search_entry().connect_search_changed(
            clone!(@weak explore_view => move |entry| {
                if entry.text().len() > 0 {
                    explore_view.load_search_results(&entry.text().to_string());
                }
            }),
        );

        explore_view.search_results().connect_child_activated(
            clone!(
                @weak self as window,
                @weak explore_view,
                @weak preview => move |_list, child| {
                let index: i32 = child.index();

                if let Some(search_result) = explore_view.search_result_at_index(&index) {
                    window.switch_view(View::Preview);
                    preview.load_podcast(&search_result)
                }
            }),
        );
    }

    fn return_from_explore(&self) {
        let (sender, receiver) = async_channel::bounded(1);

        runtime().spawn(clone!(@strong sender => async move {
            let podcast_count = podcast::repository::load_count().await;

            sender.send(podcast_count).await.unwrap();
        }));

        glib::spawn_future_local(
            clone!(@weak self as window, @strong receiver => async move {
                while let Ok(podcast_count) = receiver.recv().await {
                    if podcast_count > 0 {
                        window.switch_view(View::Podcasts);
                    } else {
                        window.switch_view(View::Empty);
                    }
                }
            }),
        );
    }
}
