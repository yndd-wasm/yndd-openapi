use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use serde_json::Value;

/// TransformRequestSpec struct
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(group = "app.yndd.io", version = "v1alpha1", kind = "TransformResponse", plural = "transformresponses")]
#[kube(namespaced)]
pub struct TransformResponseSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<TransformResponseResult>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TransformResponseResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
}

