use holochain_core_types::{
    cas::content::{AddressableContent, Content},
    json::JsonString,
};
use nucleus::ribosome::api::Runtime;
use serde_json;
use wasmi::{RuntimeArgs, RuntimeValue, Trap};

#[derive(Serialize)]
struct InitGlobalsOutput {
    app_name: String,
    app_dna_hash: String,
    app_agent_id_entry_content: Content,
    app_agent_key_hash: String,
    app_agent_initial_hash: String,
    app_agent_latest_hash: String,
}

impl From<InitGlobalsOutput> for JsonString {
    fn from(init_globals_output: InitGlobalsOutput) -> JsonString {
        JsonString::from(
            serde_json::to_string(&init_globals_output)
                .expect("could not serialize init globals output"),
        )
    }
}

/// ZomeApiFunction::InitGlobals secret function code
/// args: [0] encoded MemoryAllocation as u32
/// Not expecting any complex input
/// Returns an HcApiReturnCode as I32
pub fn invoke_init_globals(
    runtime: &mut Runtime,
    _args: &RuntimeArgs,
) -> Result<Option<RuntimeValue>, Trap> {
    let globals = InitGlobalsOutput {
        app_name: runtime.app_name.to_string(),

        // TODO #232 - Implement Dna hash
        app_dna_hash: "FIXME-app_dna_hash".to_string(),

        app_agent_id_entry_content: runtime.context.agent_id_entry().content(),

        // TODO #233 - Implement agent pub key hash
        app_agent_key_hash: "FIXME-app_agent_key_hash".to_string(),

        // TODO #234 - Implement agent identity entry hashes
        app_agent_initial_hash: "FIXME-app_agent_initial_hash".to_string(),
        app_agent_latest_hash: "FIXME-app_agent_latest_hash".to_string(),
    };

    runtime.store_utf8(JsonString::from(globals))
}

#[cfg(test)]
pub mod tests {
    use nucleus::ribosome::{
        api::{tests::test_zome_api_function_runtime, ZomeApiFunction},
        Defn,
    };

    #[test]
    /// test that bytes passed to debug end up in the log
    fn test_init_globals() {
        let input: Vec<u8> = vec![];
        let (runtime, _) =
            test_zome_api_function_runtime(ZomeApiFunction::InitGlobals.as_str(), input);
        assert_eq!(
      runtime.result.to_string(),
      "{\"app_name\":\"TestApp\",\"app_dna_hash\":\"FIXME-app_dna_hash\",\"app_agent_id_entry_content\":\"{\\\"AgentId\\\":{\\\"signing_key\\\":{},\\\"encryption_key\\\":{},\\\"dpki_root\\\":{}}}\",\"app_agent_key_hash\":\"FIXME-app_agent_key_hash\",\"app_agent_initial_hash\":\"FIXME-app_agent_initial_hash\",\"app_agent_latest_hash\":\"FIXME-app_agent_latest_hash\"}\u{0}"
        .to_string());
    }
}
