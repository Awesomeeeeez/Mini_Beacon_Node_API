use sha2::{Sha256, Digest};

use crate::models::Block;

pub fn hash_generate(
    last_block: &Block,
) -> String
{
    let mut hash = Sha256::new();

    hash.update(last_block.slot.to_le_bytes());
    hash.update(last_block.root.as_bytes());
    hash.update(last_block.parent_root.as_bytes());
    hash.update(last_block.proposer_index.to_le_bytes());
    hash.update(last_block.body.as_bytes());

    format!("0x{}", hex::encode(hash.finalize()))
}