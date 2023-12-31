pub mod connection;
pub mod episode;
pub mod episode_client;
pub mod recent_episodes;

use once_cell::sync::Lazy;
use ureq::{Agent, AgentBuilder};

pub static AGENT: Lazy<Agent> = Lazy::new(|| AgentBuilder::new().build());
