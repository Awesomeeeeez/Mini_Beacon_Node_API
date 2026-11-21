use crate::{models::{ApiError, Block, NewBlock}};

pub fn validate_new_block(
    last_block: &Block,
    payload: &NewBlock,
) -> Result<(), ApiError>
{
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

    Ok(())
}