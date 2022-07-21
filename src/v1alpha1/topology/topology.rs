use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// TopologyDefinitionSpec struct
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "topo.yndd.io",
    version = "v1alpha1",
    kind = "Topology",
    plural = "topologies"
)]
#[kube(namespaced)]
#[kube(status = "TopologyStatus")]
pub struct TopologySpec {
    /// Lifecycle determines the deletion and deployment lifecycle policies the resource will follow
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<TopologyLifecycle>,
    /// Properties define the properties of the Topology
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TopologyProperties>,
    /// TargetReference specifies which target will be used to perform crud operations for the managed resource
    #[serde(rename = "targetRef")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_ref: Option<TopologyTargetRef>,
}

/// Lifecycle determines the deletion and deployment lifecycle policies the resource will follow
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TopologyLifecycle {
    /// DeletionPolicy specifies what will happen to the underlying external when this managed resource is deleted - either "delete" or "orphan" the external resource.
    #[serde(rename = "deletionPolicy")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deletion_policy: Option<String>,
    /// Active specifies if the managed resource is active or plannned
    #[serde(rename = "deploymentPolicy")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployment_policy: Option<String>,
}

/// Properties define the properties of the Topology
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TopologyProperties {
    /// TopologySpecDefaults struct
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub defaults: Option<TopologyPropertiesDefaults>,
    #[serde(rename = "vendorTypeInfo")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vendor_type_info: Option<Vec<TopologyPropertiesVendorTypeInfo>>,
}

/// TopologySpecDefaults struct
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TopologyPropertiesDefaults {
    #[serde(rename = "expectedSwVersion")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_sw_version: Option<String>,
    #[serde(rename = "macAddress")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[serde(rename = "mgmtIPAddress")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mgmt_ip_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(rename = "serialNumber")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<BTreeMap<String, String>>,
    #[serde(rename = "vendorType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vendor_type: Option<String>,
}

/// NodeProperties struct
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TopologyPropertiesVendorTypeInfo {
    #[serde(rename = "expectedSwVersion")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_sw_version: Option<String>,
    #[serde(rename = "macAddress")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[serde(rename = "mgmtIPAddress")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mgmt_ip_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(rename = "serialNumber")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<BTreeMap<String, String>>,
    #[serde(rename = "vendorType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vendor_type: Option<String>,
}

/// TargetReference specifies which target will be used to perform crud operations for the managed resource
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TopologyTargetRef {
    /// Name of the referenced object.
    pub name: String,
}

/// A TopologyStatus represents the observed state of a Topology.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TopologyStatus {
    /// Conditions of the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<TopologyStatusConditions>>,
    /// the health condition status
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health: Option<TopologyStatusHealth>,
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
pub struct TopologyStatusConditions {
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
pub struct TopologyStatusHealth {
    /// HealthConditions that determine the health status.
    #[serde(rename = "healthConditions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health_conditions: Option<Vec<TopologyStatusHealthHealthConditions>>,
    /// LastTransitionTime is the last time this condition transitioned from one status to another.
    #[serde(rename = "lastTransitionTime")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_transition_time: Option<String>,
    /// Status of the health in percentage
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percentage: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TopologyStatusHealthHealthConditions {
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
