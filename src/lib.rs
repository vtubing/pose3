mod group;
mod r#type;

pub use group::Group;
pub use r#type::Type;

#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
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
