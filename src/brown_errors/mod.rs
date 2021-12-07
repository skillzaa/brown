use thiserror::Error;

#[derive(Error, Debug)]
pub enum BrownErrors {
    #[error("path must begin with alphabet, remove ./ if any in the start")]
    PathBeginWithAlphabet,
}