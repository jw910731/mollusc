//! Errors for `llvm-mapper`.

use llvm_bitstream::error::Error as BitstreamError;
use thiserror::Error as ThisError;

use crate::block::BlockId;
use crate::record::DataLayoutParseError;

/// All possible errors that can occur while mapping a bitstream.
#[derive(Debug, ThisError)]
pub enum Error {
    /// We encountered an error while performing the underlying bitstream parse.
    #[error("error while parsing the bitstream: {0}")]
    Parse(#[from] BitstreamError),
    /// We couldn't unroll the stream because of a structural error.
    #[error("error while unrolling the bitstream: {0}")]
    BadUnroll(String),
    /// We couldn't map a record, for any number of reasons.
    #[error("error while mapping record: {0}")]
    BadRecordMap(String),
    /// We couldn't map a block, for any number of reasons.
    #[error("error while mapping block: {0}")]
    BadBlockMap(String),
    /// We couldn't interpret a record field, for any number of reasons.
    #[error("error while decoding record field: {0}")]
    BadField(String),
    /// We expected exactly one record with this code in this block.
    #[error("expected exactly one record of code {0} in block {1:?}")]
    BlockRecordMismatch(u64, BlockId),
    /// We expected exactly one sub-block with this ID in this block.
    #[error("expected exactly one block of ID {0:?} in block {1:?}")]
    BlockBlockMismatch(BlockId, BlockId),
    /// We couldn't parse the datalayout specifier.
    #[error("error while parsing datalayout: {0}")]
    BadDataLayout(#[from] DataLayoutParseError),
    /// The bitstream contains features or is represented in a way we don't support. Yet.
    #[error("unsupported: {0}")]
    Unsupported(String),
}
