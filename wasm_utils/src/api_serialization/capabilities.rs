use holochain_core_types::{
    cas::content::Address,
    entry::cap_entries::{CapFunctions, CapabilityType},
    error::HolochainError,
    json::*,
};

// arguments required for calling grant_capability
#[derive(Deserialize, Default, Debug, Serialize, DefaultJson)]
pub struct GrantCapabilityArgs {
    pub id: String,
    pub cap_type: CapabilityType,
    pub assignees: Option<Vec<Address>>,
    pub functions: CapFunctions,
}
