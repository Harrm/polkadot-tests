use crate::host_api::utils::{ParsedInput, Runtime};
use parity_scale_codec::{Encode, Decode};

#[derive(Debug, Decode)]
struct OpaqueNetworkState {
	_peer_id: Vec<u8>,
	_addresses: Vec<Vec<u8>>,
}

pub fn ext_offchain_is_validator_version_1(mut rtm: Runtime, _input: ParsedInput) {
	let res = rtm.call_and_decode::<bool>(
		"rtm_ext_offchain_is_validator_version_1", &[]
	);

	// Always returns `true`.
	println!("{}", res);
}

pub fn ext_offchain_submit_transaction_version_1(_rtm: Runtime, _input: ParsedInput) {

}

pub fn ext_offchain_network_state_version_1(mut rtm: Runtime, _input: ParsedInput) {
	let res = rtm.call_and_decode::<Result<OpaqueNetworkState, ()>>(
		"rtm_ext_offchain_is_validator_version_1", &[]
	);

	// Always returns `Err(())`.
	println!("{:?}", res);
}

pub fn ext_offchain_timestamp_version_1(mut rtm: Runtime, _input: ParsedInput) {
	let res = rtm.call_and_decode::<u64>(
		"rtm_ext_offchain_timestamp_version_1", &[]
	);

	// Always returns `0`.
	println!("{:?}", res);
}

pub fn ext_offchain_sleep_until_version_1(mut rtm: Runtime, _input: ParsedInput) {
	use std::time::{SystemTime, UNIX_EPOCH};

	// Sleep for 5 seconds.
    let deadline = SystemTime::now()
        .duration_since(UNIX_EPOCH)
		.unwrap()
		.as_millis()
		+ 5_000;

	let _res = rtm.call_and_decode::<()>(
		"rtm_ext_offchain_sleep_until_version_1", &(deadline).encode()
	);
}

pub fn ext_offchain_random_seed_version_1(_rtm: Runtime, _input: ParsedInput) {
	unimplemented!()
}

pub fn ext_offchain_local_storage_set_version_1(_rtm: Runtime, _input: ParsedInput) {
	unimplemented!()
}

pub fn ext_offchain_local_storage_clear_version_1(_rtm: Runtime, _input: ParsedInput) {
	unimplemented!()
}

pub fn ext_offchain_local_storage_compare_and_set_version_1(_rtm: Runtime, _input: ParsedInput) {
	unimplemented!()
}

pub fn ext_offchain_local_storage_get_version_1(_rtm: Runtime, _input: ParsedInput) {
	unimplemented!()
}

pub fn ext_offchain_http_request_start_version_1(_rtm: Runtime, _input: ParsedInput) {
	unimplemented!()
}

pub fn ext_offchain_http_request_add_header_version_1(_rtm: Runtime, _input: ParsedInput) {
	unimplemented!()
}

pub fn ext_offchain_http_request_write_body_version_1(_rtm: Runtime, _input: ParsedInput) {
	unimplemented!()
}

pub fn ext_offchain_http_response_wait_version_1(_rtm: Runtime, _input: ParsedInput) {
	unimplemented!()
}

pub fn ext_offchain_http_response_headers_version_1(_rtm: Runtime, _input: ParsedInput) {
	unimplemented!()
}

pub fn ext_offchain_http_response_read_body_version_1(_rtm: Runtime, _input: ParsedInput) {
	unimplemented!()
}
