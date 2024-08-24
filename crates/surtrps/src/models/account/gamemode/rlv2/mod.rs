use serde::{Deserialize, Serialize};

pub mod current;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Rlv2 {}
