use candid::{CandidType, Deserialize};
use ic_cdk::export_candid;
use ic_cdk_macros::{init, query, update};
use std::collections::HashMap;

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

static mut STATE: Option<ContractState> = None;

#[init]
fn init() {
    let state = ContractState {
        users: HashMap::new(),
        batches: HashMap::new(),
        next_nft_id: 1,
    };
    unsafe {
        STATE = Some(state);
    }
}

fn only_authenticated() -> PrincipalId {
    let caller = ic_cdk::caller().to_string();
    let state = unsafe { STATE.as_ref().unwrap() };

    if state.users.contains_key(&caller) {
        caller
    } else {
        ic_cdk::trap("Unauthorized access");
    }
}

#[update]
fn add_user(principal: PrincipalId, is_producer: bool, is_wholesaler: bool) {
    let caller = only_authenticated();
    let mut state = unsafe { STATE.as_mut().unwrap() };
    
    if state.users.get(&caller).unwrap().is_producer {
        state.users.insert(principal.clone(), User {
            principal,
            is_producer,
            is_wholesaler,
        });
    } else {
        ic_cdk::trap("Only producers can add new users");
    }
}

#[update]
fn create_batch_nft(metadata: String) -> NFTId {
    let caller = only_authenticated();
    let mut state = unsafe { STATE.as_mut().unwrap() };

    let nft_id = state.next_nft_id;
    state.next_nft_id += 1;

    let batch_nft = NFT {
        id: nft_id,
        owner: caller.clone(),
        metadata: metadata.clone(),
    };

    state.batches.insert(nft_id, Batch {
        id: nft_id,
        owner: caller.clone(),
        nfts: vec![batch_nft.clone()],
    });

    nft_id
}

#[update]
fn create_product_nft(batch_id: NFTId, metadata: String) -> NFTId {
    let caller = only_authenticated();
    let mut state = unsafe { STATE.as_mut().unwrap() };

    let batch = state.batches.get_mut(&batch_id).expect("Batch not found");
    if batch.owner != caller {
        ic_cdk::trap("You are not the owner of this batch");
    }

    let nft_id = state.next_nft_id;
    state.next_nft_id += 1;

    let product_nft = NFT {
        id: nft_id,
        owner: caller.clone(),
        metadata: metadata.clone(),
    };

    batch.nfts.push(product_nft.clone());

    nft_id
}

#[update]
fn transfer_nft_to_wholesaler(batch_id: NFTId, nft_id: NFTId, wholesaler_principal: PrincipalId) {
    let caller = only_authenticated();
    let mut state = unsafe { STATE.as_mut().unwrap() };

    let batch = state.batches.get_mut(&batch_id).expect("Batch not found");
    if !batch.nfts.iter().any(|nft| nft.id == nft_id && nft.owner == caller) {
        ic_cdk::trap("NFT not owned by caller or does not exist in this batch");
    }

    let nft = batch.nfts.iter_mut().find(|nft| nft.id == nft_id).unwrap();
    if state.users.get(&wholesaler_principal).unwrap().is_wholesaler {
        nft.owner = wholesaler_principal;
    } else {
        ic_cdk::trap("Recipient is not a wholesaler");
    }
}

#[query]
fn get_nft_info(batch_id: NFTId, nft_id: NFTId) -> Option<NFT> {
    let state = unsafe { STATE.as_ref().unwrap() };

    let batch = state.batches.get(&batch_id)?;
    batch.nfts.iter().find(|nft| nft.id == nft_id).cloned()
}

export_candid!();