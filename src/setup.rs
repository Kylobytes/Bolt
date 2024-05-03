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

use std::path::Path;

use anyhow::Result;
use directories::ProjectDirs;

pub fn init_project_dirs() -> Result<()> {
    let project_dirs: Option<ProjectDirs> =
        ProjectDirs::from("com", "Kylobytes", "Bolt");

    if let Some(project_dirs) = project_dirs {
        let data_dir: &Path = project_dirs.data_dir();

        if !data_dir.exists() {
            std::fs::create_dir_all(data_dir)?;
        }
    }

    Ok(())
}
