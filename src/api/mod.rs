use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub mod bevy_half;
pub mod coms;
pub mod http;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum BevyApiResponse {}

#[derive(Clone, Debug, Deserialize, Serialize, Event)]
pub enum BevyApiCommand {}
