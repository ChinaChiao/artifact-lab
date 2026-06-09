pub mod dto;
pub mod error;
mod generator;
pub mod genshin;
pub mod graduation;
pub mod hsr;
pub mod meta;
pub mod rng;
pub mod rules;

pub use dto::{ItemDTO, SubstatDTO};
pub use error::ArtifactError;
pub use genshin::{GenshinArtifact, GenshinGenerator};
pub use graduation::{simulate_graduation, summarize_graduation_average, GraduationConfig};
pub use hsr::{HsrGenerator, HsrRelic};
pub use meta::get_meta;
