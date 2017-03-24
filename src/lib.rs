#[macro_use]
extern crate log;

mod commands;
mod services;

use commands::{Command, CommandExecutor};
use std::error;

pub struct SovrinClient {
    command_executor: CommandExecutor
}

impl SovrinClient {
    pub fn new() -> SovrinClient {
        SovrinClient {
            command_executor: CommandExecutor::new()
        }
    }

    pub fn set_did(&self, did: String, cb: Box<Fn(Result<(), Box<error::Error>>) + Send>) {
        self.command_executor.send(Command::SetDidCommand(did, cb));
    }

    pub fn create_master_secret() {

    }

    pub fn create_keys(schema: String) {

    }

    pub fn create_context_attribute(i_a: String, user_id: String) {

    }

    pub fn issue_accumulator(schema: String, i_a: String, l: String,
                             public_key_revocation: String) {

    }

    pub fn issue_claim(attributes: String, accumulator: String,i_a: String, i: String,
                       claim_request: String, context_attribute: String, public_key: String,
                       secret_key: String, public_key_revocation: String,
                       secret_key_revocation: String, tails: String,
                       secret_key_accumulator: String) {

    }

    pub fn create_claim_request(master_secret: String, public_key: String,
                                public_key_revocation: String, request_non_revocation: String) {

    }

    pub fn create_proof(proof_input: String, nonce: String, claims: String,
                        public_key_revocation: String, accum: String, public_key: String,
                        master_secret: String) {

    }

    pub fn verify_proof(proof_input: String, proof: String, revealed_attributes: String,
                        nonce: String, public_key_revocation: String,
                        public_key_accumulator: String, accumulator: String,
                        public_key: String, attributes: String) {

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::channel;

    #[test]
    fn sovrin_client_can_be_created() {
        let sovrin_client = SovrinClient::new();
        assert!(true, "No crashes on SovrinClient::new");
    }

    #[test]
    fn sovrin_client_can_be_dropped() {
        fn drop_test() {
            let sovrin_client = SovrinClient::new();
        }

        drop_test();
        assert!(true, "No crashes on SovrinClient::drop");
    }

    #[test]
    fn set_did_method_can_be_called() {
        let (sender, receiver) = channel();

        let cb = Box::new(move |result| {
            match result {
                Ok(val) => sender.send("OK"),
                Err(err) => sender.send("ERROR")
            };
        });

        let sovrin_client = SovrinClient::new();
        sovrin_client.set_did("DID0".to_string(), cb);

        match receiver.recv() {
            Ok(result) => {
                assert_eq!("OK", result);
            }
            Err(err) => {
                panic!("Error on result recv: {:?}", err);
            }
        }
    }
}