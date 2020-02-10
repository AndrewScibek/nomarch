use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Pipeline {
    pub name: String,
    pub max_seconds_to_reach_end: u64,
    pub services: Vec<Service>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Service {
    pub name: String,
    pub children: Vec<String>,

    #[serde(default)]
    stats: Stats,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Stats {
    events_seen: f64,
    events_expected: f64,
}
