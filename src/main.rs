/* setup.rs
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

use cosmic::app::Settings;
use log::error;

use crate::application::Application;

mod application;
mod setup;

#[tokio::main]
async fn main() -> cosmic::iced::Result {
    if let Err(msg) = setup::init_project_dirs() {
        error!("Failed to initialize project directories. {}", msg);
    }

    cosmic::app::run::<Application>(Settings::default(), ())
}
