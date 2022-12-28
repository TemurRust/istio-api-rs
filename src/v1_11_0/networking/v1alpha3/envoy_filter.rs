// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium -Af resources/istio/v1_11_0/networking/v1alpha3/EnvoyFilter.yaml --api-version v1alpha3
// kopium version: 0.14.0

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

/// Customizing Envoy configuration generated by Istio. See more details at: https://istio.io/docs/reference/config/networking/envoy-filter.html
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(group = "networking.istio.io", version = "v1alpha3", kind = "EnvoyFilter", plural = "envoyfilters")]
#[kube(namespaced)]
pub struct EnvoyFilterSpec {
    /// One or more patches with match conditions.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configPatches")]
    pub config_patches: Option<Vec<EnvoyFilterConfigPatches>>,
    /// Priority defines the order in which patch sets are applied within a context.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "workloadSelector")]
    pub workload_selector: Option<EnvoyFilterWorkloadSelector>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct EnvoyFilterConfigPatches {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "applyTo")]
    pub apply_to: Option<EnvoyFilterConfigPatchesApplyTo>,
    /// Match on listener/route configuration/cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "match")]
    pub r#match: Option<EnvoyFilterConfigPatchesMatch>,
    /// The patch to apply along with the operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patch: Option<EnvoyFilterConfigPatchesPatch>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum EnvoyFilterConfigPatchesApplyTo {
    #[serde(rename = "INVALID")]
    Invalid,
    #[serde(rename = "LISTENER")]
    Listener,
    #[serde(rename = "FILTER_CHAIN")]
    FilterChain,
    #[serde(rename = "NETWORK_FILTER")]
    NetworkFilter,
    #[serde(rename = "HTTP_FILTER")]
    HttpFilter,
    #[serde(rename = "ROUTE_CONFIGURATION")]
    RouteConfiguration,
    #[serde(rename = "VIRTUAL_HOST")]
    VirtualHost,
    #[serde(rename = "HTTP_ROUTE")]
    HttpRoute,
    #[serde(rename = "CLUSTER")]
    Cluster,
    #[serde(rename = "EXTENSION_CONFIG")]
    ExtensionConfig,
    #[serde(rename = "BOOTSTRAP")]
    Bootstrap,
}

/// Match on listener/route configuration/cluster.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct EnvoyFilterConfigPatchesMatch {
    /// Match on envoy cluster attributes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster: Option<EnvoyFilterConfigPatchesMatchCluster>,
    /// The specific config generation context to match on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<EnvoyFilterConfigPatchesMatchContext>,
    /// Match on envoy listener attributes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listener: Option<EnvoyFilterConfigPatchesMatchListener>,
    /// Match on properties associated with a proxy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxy: Option<EnvoyFilterConfigPatchesMatchProxy>,
    /// Match on envoy HTTP route configuration attributes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "routeConfiguration")]
    pub route_configuration: Option<EnvoyFilterConfigPatchesMatchRouteConfiguration>,
}

/// Match on envoy cluster attributes.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct EnvoyFilterConfigPatchesMatchCluster {
    /// The exact name of the cluster to match.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The service port for which this cluster was generated.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "portNumber")]
    pub port_number: Option<i64>,
    /// The fully qualified service name for this cluster.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    /// The subset associated with the service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subset: Option<String>,
}

/// Match on listener/route configuration/cluster.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum EnvoyFilterConfigPatchesMatchContext {
    #[serde(rename = "ANY")]
    Any,
    #[serde(rename = "SIDECAR_INBOUND")]
    SidecarInbound,
    #[serde(rename = "SIDECAR_OUTBOUND")]
    SidecarOutbound,
    #[serde(rename = "GATEWAY")]
    Gateway,
}

/// Match on envoy listener attributes.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct EnvoyFilterConfigPatchesMatchListener {
    /// Match a specific filter chain in a listener.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "filterChain")]
    pub filter_chain: Option<EnvoyFilterConfigPatchesMatchListenerFilterChain>,
    /// Match a specific listener by its name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "portName")]
    pub port_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "portNumber")]
    pub port_number: Option<i64>,
}

/// Match a specific filter chain in a listener.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct EnvoyFilterConfigPatchesMatchListenerFilterChain {
    /// Applies only to sidecars.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "applicationProtocols")]
    pub application_protocols: Option<String>,
    /// The destination_port value used by a filter chain's match condition.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "destinationPort")]
    pub destination_port: Option<i64>,
    /// The name of a specific filter to apply the patch to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<EnvoyFilterConfigPatchesMatchListenerFilterChainFilter>,
    /// The name assigned to the filter chain.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The SNI value used by a filter chain's match condition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sni: Option<String>,
    /// Applies only to `SIDECAR_INBOUND` context.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "transportProtocol")]
    pub transport_protocol: Option<String>,
}

/// The name of a specific filter to apply the patch to.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct EnvoyFilterConfigPatchesMatchListenerFilterChainFilter {
    /// The filter name to match on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subFilter")]
    pub sub_filter: Option<EnvoyFilterConfigPatchesMatchListenerFilterChainFilterSubFilter>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct EnvoyFilterConfigPatchesMatchListenerFilterChainFilterSubFilter {
    /// The filter name to match on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Match on properties associated with a proxy.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct EnvoyFilterConfigPatchesMatchProxy {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "proxyVersion")]
    pub proxy_version: Option<String>,
}

/// Match on envoy HTTP route configuration attributes.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct EnvoyFilterConfigPatchesMatchRouteConfiguration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    /// Route configuration name to match on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Applicable only for GATEWAY context.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "portName")]
    pub port_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "portNumber")]
    pub port_number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vhost: Option<EnvoyFilterConfigPatchesMatchRouteConfigurationVhost>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct EnvoyFilterConfigPatchesMatchRouteConfigurationVhost {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Match a specific route within the virtual host.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route: Option<EnvoyFilterConfigPatchesMatchRouteConfigurationVhostRoute>,
}

/// Match a specific route within the virtual host.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct EnvoyFilterConfigPatchesMatchRouteConfigurationVhostRoute {
    /// Match a route with specific action type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<EnvoyFilterConfigPatchesMatchRouteConfigurationVhostRouteAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Match a specific route within the virtual host.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum EnvoyFilterConfigPatchesMatchRouteConfigurationVhostRouteAction {
    #[serde(rename = "ANY")]
    Any,
    #[serde(rename = "ROUTE")]
    Route,
    #[serde(rename = "REDIRECT")]
    Redirect,
    #[serde(rename = "DIRECT_RESPONSE")]
    DirectResponse,
}

/// The patch to apply along with the operation.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct EnvoyFilterConfigPatchesPatch {
    /// Determines the filter insertion order.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "filterClass")]
    pub filter_class: Option<EnvoyFilterConfigPatchesPatchFilterClass>,
    /// Determines how the patch should be applied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<EnvoyFilterConfigPatchesPatchOperation>,
    /// The JSON config of the object being patched.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<BTreeMap<String, serde_json::Value>>,
}

/// The patch to apply along with the operation.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum EnvoyFilterConfigPatchesPatchFilterClass {
    #[serde(rename = "UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "AUTHN")]
    Authn,
    #[serde(rename = "AUTHZ")]
    Authz,
    #[serde(rename = "STATS")]
    Stats,
}

/// The patch to apply along with the operation.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub enum EnvoyFilterConfigPatchesPatchOperation {
    #[serde(rename = "INVALID")]
    Invalid,
    #[serde(rename = "MERGE")]
    Merge,
    #[serde(rename = "ADD")]
    Add,
    #[serde(rename = "REMOVE")]
    Remove,
    #[serde(rename = "INSERT_BEFORE")]
    InsertBefore,
    #[serde(rename = "INSERT_AFTER")]
    InsertAfter,
    #[serde(rename = "INSERT_FIRST")]
    InsertFirst,
    #[serde(rename = "REPLACE")]
    Replace,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct EnvoyFilterWorkloadSelector {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
}

