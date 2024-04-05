/* mod.rs
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

use rss::Channel;

use crate::api::CLIENT;

pub async fn download(feed_url: &str) -> Channel {
    let content = CLIENT.get(feed_url)
        .send()
        .await
        .expect("Failed to download feed")
        .bytes()
        .await
        .expect("Failed to parse feed");

    Channel::read_from(&content[..]).expect("Failed read feed")
}
