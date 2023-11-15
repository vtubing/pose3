#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct Pose3 {
  pub fade_in_time: Option<f64>,
  #[serde(default)]
  pub groups: Vec<Vec<Group>>,
  #[serde(rename = "Type")]
  pub type_: Type,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[remain::sorted]
pub enum Type {
  #[serde(rename = "Live2D Pose")]
  Live2D,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct Group {
  pub id: String,
  pub link: Vec<String>,
}
