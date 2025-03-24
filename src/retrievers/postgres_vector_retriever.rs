use crate::clients::AsyncEmbeddingClient;
use crate::common::{Chunk, Chunks, Embedding};
use crate::retrievers::traits::AsyncRetriever;
use std::error::Error;
use std::num::NonZeroU32;
use thiserror::Error;

/// # [`DistanceFunction`]
/// This is an enum for the types of distance functions
/// that can be used to compare vectors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DistanceFunction {
    L2,
    Cosine,
    InnerProduct,
}
impl DistanceFunction {
    pub fn to_sql_string(&self) -> &str {
        match self {
            DistanceFunction::L2 => "<->",
            DistanceFunction::Cosine => "<=>",
            DistanceFunction::InnerProduct => "<#>",
        }
    }
}
