use crate::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Components {
	#[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
	pub security_schemes: BTreeMap<String, ReferenceOr<SecurityScheme>>,
	#[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
	pub responses: BTreeMap<String, ReferenceOr<Response>>,
	#[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
	pub parameters: BTreeMap<String, ReferenceOr<Parameter>>,
	#[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
	pub examples: BTreeMap<String, ReferenceOr<Example>>,
	#[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
	pub request_bodies: BTreeMap<String, ReferenceOr<RequestBody>>,
	#[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
	pub headers: BTreeMap<String, ReferenceOr<Header>>,
	#[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
	pub schemas: BTreeMap<String, ReferenceOr<Schema>>,
	#[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
	pub links: BTreeMap<String, ReferenceOr<Link>>,
	#[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
	pub callbacks: BTreeMap<String, ReferenceOr<Callback>>,
}
