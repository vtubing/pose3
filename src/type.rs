#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize)]
#[remain::sorted]
pub enum Type {
  #[serde(rename = "Live2D Pose")]
  Live2D,
}
