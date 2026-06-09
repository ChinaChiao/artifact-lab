use thiserror::Error;

#[derive(Error, Debug)]
pub enum ArtifactError {
    #[error("Invalid source: {0}")]
    InvalidSource(String),

    #[error("Invalid level: {0}")]
    InvalidLevel(i32),

    #[error("At least one set is required")]
    EmptySets,

    #[error("Item is already at max level")]
    MaxLevelReached,

    #[error("Invalid main stat for slot")]
    InvalidMainStat,

    #[error("Empty weight pool")]
    EmptyWeightPool,
}

pub type Result<T> = std::result::Result<T, ArtifactError>;
