pub mod connection;
pub mod episode;
pub mod podcast;
pub mod search;

use once_cell::sync::Lazy;
use reqwest::Client;
use ureq::{Agent, AgentBuilder};

pub static AGENT: Lazy<Agent> = Lazy::new(|| AgentBuilder::new().build());
pub static CLIENT: Lazy<Client> = Lazy::new(|| Client::new());
