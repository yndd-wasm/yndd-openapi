use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// TargetSpec struct
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "target.yndd.io",
    version = "v1",
    kind = "Target",
    plural = "targets"
)]
#[kube(namespaced)]
#[kube(status = "TargetStatus")]
pub struct TargetSpec {
    /// DiscoveryInfo
    #[serde(rename = "discoveryInfo")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discovery_info: Option<TargetDiscoveryInfo>,
    /// Properties define the properties of the Target
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<TargetProperties>,
}

/// DiscoveryInfo
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TargetDiscoveryInfo {
    /// Host name of the target
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// Last discovery time
    #[serde(rename = "lastSeen")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<String>,
    /// the Mac address of the target
    #[serde(rename = "macAddress")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    /// the Kind of the target
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// the Serial Number of the target
    #[serde(rename = "serialNumber")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// Supported Encodings of the target
    #[serde(rename = "supportedEncodings")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supported_encodings: Option<Vec<String>>,
    /// SW version of the target
    #[serde(rename = "swVersion")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sw_version: Option<String>,
    /// the VendorType of the target
    #[serde(rename = "vendorType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vendor_type: Option<String>,
}

/// Properties define the properties of the Target
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TargetProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocation: Option<BTreeMap<String, TargetPropertiesAllocation>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<TargetPropertiesConfig>,
    #[serde(rename = "vendorType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vendor_type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TargetPropertiesAllocation {
    #[serde(rename = "serviceIdentity")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_identity: Option<String>,
    #[serde(rename = "serviceName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TargetPropertiesConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "credentialName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insecure: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxy: Option<String>,
    #[serde(rename = "skipVerify")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skip_verify: Option<bool>,
    #[serde(rename = "tlsCredentialName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls_credential_name: Option<String>,
}

/// TargetStatus defines the observed state of TargetNode
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TargetStatus {
    /// Conditions of the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<TargetStatusConditions>>,
    /// identifies the controller reference of the target
    #[serde(rename = "controllerRef")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub controller_ref: Option<TargetStatusControllerRef>,
}

/// A Condition that may apply to a resource
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TargetStatusConditions {
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

/// identifies the controller reference of the target
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct TargetStatusControllerRef {
    /// Name of the referenced object.
    pub name: String,
}
