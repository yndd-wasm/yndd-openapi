use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use serde_json::Value;

/// TransformRequestSpec struct
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(group = "app.yndd.io", version = "v1alpha1", kind = "TransformRequest", plural = "transformrequests")]
#[kube(namespaced)]
pub struct TransformRequestSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<Value>,
}

