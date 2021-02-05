use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SortFile {
  pub rules: Rules
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rules {
  pub order: Option<Order>,
  pub must_exist: Option<bool>,
  pub must_be_unique: Option<bool>,
  pub must_be_absolute: Option<bool>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
  pub paths: Option<Vec<Path>>,
  pub sort: Option<Sort>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Path {
  #[serde(rename="exact")]
  Exact(String),
  #[serde(rename="contains")]
  Contains(String),
  #[serde(rename="default")]
  Default(bool)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sort {
  pub alphabetical: Option<i8>,
  pub path_component_length: Option<i8>
}
