use std::str::FromStr;
use pyo3::prelude::*;
use tokio::runtime::Runtime;
use tokio::task;

use shared_crypto::intent::Intent;
use sui_json_rpc_types::SuiTransactionBlockResponseOptions;
use sui_keys::keystore::{AccountKeystore, FileBasedKeystore, Keystore};
use sui_sdk::{
    types::{
        base_types::{ObjectID, SuiAddress},
        transaction::Transaction,
    },
    SuiClientBuilder,
};
use sui_types::quorum_driver_types::ExecuteTransactionRequestType;

async fn _sendSui(
            to: String, 
            amount: u64, 
            privateKey: String, 
            rpc: String, 
        ) -> String 
    {

    let private_key = PrivateKey::from_str(privateKey);
    let public_key: PublicKey = (&private_key).into();
    let keystore = FileBasedKeystore::new(private_key.unwrap());

    let address = SuiAddress::from(public_key);
    let to = SuiAddress::from_str(&to).unwrap();

    let gas_object_id = ObjectID::random();

    let sui = SuiClientBuilder::default()
        .build(rpc) // TODO: make optional with default value
        .await.unwrap();

    let transfer_tx = sui
        .transaction_builder()
        .transfer_sui(
                address, // sender
                gas_object_id, // gas object id. For some reason, this is required, but not validated. this is a random value. (def ln 24)
                amount, // This may be in the wrong spot. It's not documented.
                to, 
                Some(1000) // gas limit?
        ).await;
    let signature = keystore.sign_secure(&address, &transfer_tx, Intent::sui_transaction())?;

    let tx = sui
        .quorum_driver_api()
        .execute_transaction_block(
            Transaction::from_data(transfer_tx.unwrap(), Intent::sui_transaction(), vec![signature]),
            SuiTransactionBlockResponseOptions::full_content(),
            Some(ExecuteTransactionRequestType::WaitForLocalExecution),
        ).await; 

    return format!("{:?}", tx); // Return TX hash as a string for formatting 
}

#[pyfunction]
pub fn sendSui(py: Python<'_>, to: String, amount: u64, privateKey: String, rpc: String) -> String {
    let rt = Runtime::new().unwrap();
    let result = rt.block_on(async {
        let handle = task::spawn(_sendSui(to, amount, privateKey, rpc));
        handle.await.unwrap()
    });

    return format!("{:?}", result)
}