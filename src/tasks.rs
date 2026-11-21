use std::{format};
use axum::{Json, extract::{Path, Query, State}, http::StatusCode};
use sha2::{Sha256, Digest};

use crate::{models::NewBlock, state::AppState};
use crate::models::{Block, Health, RootResponse, ApiError, Pagination};

pub async fn health(
    State(state): State<AppState>,
) -> Json<Health> {
    Json(
        Health{
            status: "ok".to_string(),
            blocks_count: state.blocks.read().await.len(),
        }
    )
}

pub async fn get_one(
    State(state): State<AppState>,
    Path(root): Path<String>,
) -> Result<Json<Block>, ApiError>
{
    let list = state.blocks.read().await;

    list
        .iter()
        .find(|block| block.root == root)
        .map(|x| Json(x.clone()))
        .ok_or(ApiError::NotFound)
}

pub async fn head(
    State(state): State<AppState>,
) -> Json<Block>
{
    Json(state.last_block.read().await.clone())
}

pub async fn new_block(
    State(state): State<AppState>,
    Json(payload): Json<NewBlock>,
) -> Result<(StatusCode, Json<RootResponse>), ApiError>
{
    let last_block = state.last_block.read().await;

    if payload.parent_root.len() != 66 {
        return Err(ApiError::InvalidFormat);
    }
    if payload.parent_root != last_block.root {
        return Err(ApiError::ParentUnknown);
    }
    if payload.slot != last_block.slot + 1 {
        return Err(ApiError::OrderDisrupted);
    }
    if payload.body.len() > 1024 {
        return Err(ApiError::BadData);
    }

    let mut hash = Sha256::new();

    hash.update(last_block.slot.to_le_bytes());
    hash.update(last_block.root.as_bytes());
    hash.update(last_block.parent_root.as_bytes());
    hash.update(last_block.proposer_index.to_le_bytes());
    hash.update(last_block.body.as_bytes());

    let new_root = format!("0x{}", hex::encode(hash.finalize()));

    let new_block = 
        Block {
            slot: last_block.slot + 1,
            root: new_root.clone(),
            parent_root: last_block.root.clone(),
            proposer_index: last_block.proposer_index + payload.proposer_index,
            body: format!("{}{}", last_block.body, payload.body), 
        }
    ;

    drop(last_block);

    state.blocks.write().await.push(new_block.clone());
    
    let mut last_block = state.last_block.write().await;
    *last_block = new_block;

    Ok((
        StatusCode::OK,
        Json(
            RootResponse {
                root: new_root,
            }
        ),
    ))
}

pub async fn get_blocks(
    State(state): State<AppState>,
    Query(params): Query<Pagination>,
) -> Result<Json<Vec<Block>>, ApiError>
{
    let blocks = state.blocks.read().await;

    let from_slot = params.from_slot.unwrap_or(0);
    let mut limit = params.limit.unwrap_or(blocks.len());

    if from_slot >= blocks.len() {
        return Err(ApiError::BadData);
    }
    
    limit = 
        if from_slot + limit > blocks.len() {
            blocks.len()
        } else{
            from_slot + limit
        }
    ;

    Ok(
        Json(
            blocks[from_slot..limit].to_vec()
        )
    )
}