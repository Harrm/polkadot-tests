use super::utils::MiscApi;
use super::ParsedInput;

pub fn test_chain_id() {
    let mut api = MiscApi::new();

    let _res = api.rtm_ext_chain_id();
    // TODO...
}