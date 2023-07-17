use std::str::FromStr;
use sui_json_rpc_types::{SuiObjectDataOptions, SuiObjectResponseQuery, SuiObjectResponse};
use sui_sdk::SuiClientBuilder;
use sui_sdk::types::base_types::SuiAddress;
use pyo3::prelude::*;
use tokio::runtime::Runtime;
use tokio::task;

async fn _getOwnedObjects(address: String, rpc: String) -> Vec<SuiObjectResponse> {
    let sui = SuiClientBuilder::default()
        .build(rpc)
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
pub fn getOwnedObjects(py: Python<'_>, address: String, rpc: String) -> String {
    let rt = Runtime::new().unwrap();
    let result = rt.block_on(async {
        let handle = task::spawn(_getOwnedObjects(address, rpc));
        handle.await.unwrap()
    });

    format!("{:?}", result)
}