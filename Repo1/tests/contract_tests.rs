use ic_cdk::api::call::CallResult;
use ic_cdk::api::call::call;
use ic_cdk::api::caller;
use candid::{CandidType, Deserialize};
use std::collections::HashMap;

// Define the types used in the contract.
type PrincipalId = String;
type NFTId = u64;

#[derive(Clone, CandidType, Deserialize)]
struct NFT {
    id: NFTId,
    owner: PrincipalId,
    metadata: String,
}

#[derive(Clone, CandidType, Deserialize)]
struct Batch {
    id: NFTId,
    owner: PrincipalId,
    nfts: Vec<NFT>,
}

#[derive(Clone, CandidType, Deserialize)]
struct User {
    principal: PrincipalId,
    is_producer: bool,
    is_wholesaler: bool,
}

#[derive(Clone, CandidType, Deserialize)]
struct ContractState {
    users: HashMap<PrincipalId, User>,
    batches: HashMap<NFTId, Batch>,
    next_nft_id: NFTId,
}

// Test initialization
#[test]
fn test_init() {
    // Call the init function
    let result: Result<(), String> = call(
        "init",
        (),
    ).await;

    assert_eq!(result, Ok(()));
}

// Test adding a user
#[test]
fn test_add_user() {
    // Set up initial state
    let result: Result<(), String> = call(
        "add_user",
        ("user1".to_string(), true, false),
    ).await;

    assert_eq!(result, Ok(()));
}

// Test creating a batch NFT
#[test]
fn test_create_batch_nft() {
    // Set up initial state
    let result: Result<NFTId, String> = call(
        "create_batch_nft",
        "metadata".to_string(),
    ).await;

    assert!(result.is_ok());
    let nft_id = result.unwrap();
    assert!(nft_id > 0);
}

// Test creating a product NFT
#[test]
fn test_create_product_nft() {
    // Set up initial state
    let batch_id = 1; // Assume batch with ID 1 exists
    let result: Result<NFTId, String> = call(
        "create_product_nft",
        (batch_id, "product_metadata".to_string()),
    ).await;

    assert!(result.is_ok());
    let nft_id = result.unwrap();
    assert!(nft_id > 0);
}

// Test transferring NFT to wholesaler
#[test]
fn test_transfer_nft_to_wholesaler() {
    // Set up initial state
    let batch_id = 1; // Assume batch with ID 1 exists
    let nft_id = 1; // Assume NFT with ID 1 exists
    let wholesaler_principal = "wholesaler1".to_string();

    let result: Result<(), String> = call(
        "transfer_nft_to_wholesaler",
        (batch_id, nft_id, wholesaler_principal),
    ).await;

    assert_eq!(result, Ok(()));
}

// Test querying NFT info
#[test]
fn test_get_nft_info() {
    // Set up initial state
    let batch_id = 1; // Assume batch with ID 1 exists
    let nft_id = 1; // Assume NFT with ID 1 exists

    let result: Result<Option<NFT>, String> = call(
        "get_nft_info",
        (batch_id, nft_id),
    ).await;

    assert!(result.is_ok());
    let nft_info = result.unwrap();
    assert!(nft_info.is_some());
    let nft = nft_info.unwrap();
    assert_eq!(nft.id, nft_id);
}
