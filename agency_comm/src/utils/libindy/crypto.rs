use indy::crypto;
use futures::Future;
use crate::utils::error::VcxResult;

pub fn pack_message(sender_vk: Option<&str>, receiver_keys: &str, msg: &[u8]) -> VcxResult<Vec<u8>> {
    if ::std::env::var("DUMMY_TEST_MODE").unwrap_or("false".to_string()) == "true" { return Ok(msg.to_vec()); } 

    crypto::pack_message(crate::utils::wallet::get_wallet_handle(), msg, receiver_keys, sender_vk)
        .wait()
        .map_err(|err| err.into())
}

pub fn unpack_message(msg: &[u8]) -> VcxResult<Vec<u8>> {
    if ::std::env::var("DUMMY_TEST_MODE").unwrap_or("false".to_string()) == "true" { return Ok(msg.to_vec()); }

    crypto::unpack_message(crate::utils::wallet::get_wallet_handle(), msg)
        .wait()
        .map_err(|err| err.into())
}