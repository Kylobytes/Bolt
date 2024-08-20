/* about.rs
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

use gtk::prelude::GtkWindowExt;
use relm4::{adw, gtk, ComponentParts, ComponentSender, SimpleComponent};

use crate::config::{APP_ID, VERSION};

pub struct AboutDialog {}

impl SimpleComponent for AboutDialog {
    type Init = ();
    type Widgets = adw::AboutWindow;
    type Input = ();
    type Output = ();
    type Root = adw::AboutWindow;

    fn init_root() -> Self::Root {
        adw::AboutWindow::builder()
            .application_icon(APP_ID)
            .license_type(gtk::License::Gpl30)
            .website("https://github.com/Kylobytes/Bolt/")
            .issue_url("https://github.com/Kylobytes/Bolt/issues")
            .application_name("Bolt")
            .version(VERSION)
            .translator_credits("translator-credits")
            .copyright("© 2023 Kent Delante")
            .developers(vec!["Kent Delante"])
            .designers(vec!["Kent Delante"])
            .hide_on_close(true)
            .build()
    }

    fn init(
        _: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {};

        let widgets = root.clone();

        ComponentParts { model, widgets }
    }

    fn update_view(
        &self,
        dialog: &mut Self::Widgets,
        _sender: ComponentSender<Self>,
    ) {
        dialog.present();
    }
}
