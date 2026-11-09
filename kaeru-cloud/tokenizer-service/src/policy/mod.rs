mod evaluator;
mod validation;

pub use evaluator::{Evaluator, check_shape, purpose_allows};
pub use validation::{normalize_changes, validate_snapshot, validate_transition};
