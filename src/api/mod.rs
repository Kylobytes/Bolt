pub mod connection;
pub mod episode;
pub mod podcast;
pub mod search;

use once_cell::sync::Lazy;
use reqwest::Client;

pub static CLIENT: Lazy<Client> = Lazy::new(|| Client::new());
