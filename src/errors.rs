use thiserror::Error;

#[derive(Error, Clone, Debug)]
pub enum MathError{
    #[error("Invalid bounds for root-finding.")]
    BoundsError,
}