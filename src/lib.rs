use std::str::FromStr;
use sui_json_rpc_types::{SuiObjectDataOptions, SuiObjectResponseQuery, SuiObjectResponse};
use sui_sdk::SuiClientBuilder;
use sui_sdk::types::base_types::SuiAddress;
use pyo3::prelude::*;
use tokio::runtime::Runtime;
use tokio::task;
// use ed25519_dalek::{Ed25519PrivateKey, Ed25519PublicKey};

// let private_key = Ed25519PrivateKey::from_str(privateKey)?;
// let public_key: Ed25519PublicKey = (&private_key).into();
// let address = SuiAddress::from(public_key);

async fn _getOwnedObjects(address: String) -> Vec<SuiObjectResponse> {
    let sui = SuiClientBuilder::default()
        .build("https://rpc.devnet.sui.io:443")
        .await.unwrap();
    let address = SuiAddress::from_str(&address).unwrap();
    let objects = sui
        .read_api()
        .get_owned_objects(
            address,
            Some(SuiObjectResponseQuery::new_with_options(
                SuiObjectDataOptions::new(),
            )),
            None,
            None,
        )
        .await;
    let output = &objects.unwrap().data;
    println!("{:?}", output);
    return output.to_vec();
}

#[pyfunction]
fn getOwnedObjects(py: Python<'_>, address: String) -> String {
    let mut rt = Runtime::new().unwrap();
    let result = rt.block_on(async {
        let handle = task::spawn(_getOwnedObjects(address));
        handle.await.unwrap()
    });

    format!("{:?}", result)
}

// async fn sendSui(to: String, from: String, amount: u64, privateKey: String) {
//     let sui = SuiClientBuilder::default()
//         .build("https://rpc.devnet.sui.io:443") // TODO: Change to mainnet
//         .await.unwrap();
//     let from = SuiAddress::from_str(&from).unwrap(); // Not sure why this is wrapped.
//     let to = SuiAddress::from_str(&to).unwrap();
//     let private_key = Ed25519PrivateKey::from_str(&privateKey).unwrap();
//     let public_key: Ed25519PublicKey = (&private_key).into();
//     let address = SuiAddress::from(public_key);
//     let tx = sui
//         .transaction_api()
//         .send_sui(
//             from,
//             to,
//             amount,
//             private_key,
//             None,
//             None,
//             None,
//             None,
//             None,
//             None,
//         )
//         .await.unwrap();
//     println!("Transaction: {:?}", tx);
// }

#[pymodule]
fn suipy(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(getOwnedObjects, m)?)?;
    Ok(())
}