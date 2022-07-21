use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// DefinitionSpec struct
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "topo.yndd.io",
    version = "v1alpha1",
    kind = "Definition",
    plural = "definitions"
)]
#[kube(namespaced)]
#[kube(status = "DefinitionStatus")]
pub struct DefinitionSpec {
    /// Lifecycle determines the deletion and deployment lifecycle policies the resource will follow
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<DefinitionLifecycle>,
    /// Properties define the properties of the Definition
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DefinitionProperties>,
    /// TargetReference specifies which target will be used to perform crud operations for the managed resource
    #[serde(rename = "targetRef")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_ref: Option<DefinitionTargetRef>,
}

/// Lifecycle determines the deletion and deployment lifecycle policies the resource will follow
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct DefinitionLifecycle {
    /// DeletionPolicy specifies what will happen to the underlying external when this managed resource is deleted - either "delete" or "orphan" the external resource.
    #[serde(rename = "deletionPolicy")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deletion_policy: Option<String>,
    /// Active specifies if the managed resource is active or plannned
    #[serde(rename = "deploymentPolicy")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployment_policy: Option<String>,
}

/// Properties define the properties of the Definition
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct DefinitionProperties {
    #[serde(rename = "discoveryRules")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discovery_rules: Option<Vec<DefinitionPropertiesDiscoveryRules>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub templates: Option<Vec<DefinitionPropertiesTemplates>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct DefinitionPropertiesDiscoveryRules {
    #[serde(rename = "digitalTwin")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digital_twin: Option<bool>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct DefinitionPropertiesTemplates {
    #[serde(rename = "digitalTwin")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digital_twin: Option<bool>,
    pub name: String,
}

/// TargetReference specifies which target will be used to perform crud operations for the managed resource
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct DefinitionTargetRef {
    /// Name of the referenced object.
    pub name: String,
}

/// A DefinitionStatus represents the observed state of a Definition.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct DefinitionStatus {
    /// Conditions of the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<DefinitionStatusConditions>>,
    /// the health condition status
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health: Option<DefinitionStatusHealth>,
    /// Oda []Tag `json:"oda,omitempty"`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oda: Option<BTreeMap<String, String>>,
    /// rootPaths define the rootPaths of the cr, used to monitor the resource status
    #[serde(rename = "rootPaths")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root_paths: Option<Vec<String>>,
}

/// A Condition that may apply to a resource
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct DefinitionStatusConditions {
    /// Type of this condition. At most one of each condition type may apply to a resource at any point in time.
    pub kind: String,
    /// LastTransitionTime is the last time this condition transitioned from one status to another.
    #[serde(rename = "lastTransitionTime")]
    pub last_transition_time: String,
    /// A Message containing details about this condition's last transition from one status to another, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// A Reason for this condition's last transition from one status to another.
    pub reason: String,
    /// Status of this condition; is it currently True, False, or Unknown?
    pub status: String,
}

/// the health condition status
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct DefinitionStatusHealth {
    /// HealthConditions that determine the health status.
    #[serde(rename = "healthConditions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health_conditions: Option<Vec<DefinitionStatusHealthHealthConditions>>,
    /// LastTransitionTime is the last time this condition transitioned from one status to another.
    #[serde(rename = "lastTransitionTime")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<String>,
    /// Status of the health in percentage
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percentage: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct DefinitionStatusHealthHealthConditions {
    #[serde(rename = "healthKind")]
    pub health_kind: String,
    /// LastTransitionTime is the last time this condition transitioned from one status to another.
    #[serde(rename = "lastTransitionTime")]
    pub last_transition_time: String,
    /// A Message containing details about this condition's last transition from one status to another, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// A Reason for this condition's last transition from one status to another.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Kind of this condition. At most one of each condition kind may apply to a resource at any point in time.
    #[serde(rename = "resourceName")]
    pub resource_name: String,
    /// Status of this condition; is it currently True, False, or Unknown?
    pub status: String,
}
