pub mod connection;
pub mod episode;

use once_cell::sync::Lazy;
use ureq::{Agent, AgentBuilder};

pub static AGENT: Lazy<Agent> = Lazy::new(|| AgentBuilder::new().build());
