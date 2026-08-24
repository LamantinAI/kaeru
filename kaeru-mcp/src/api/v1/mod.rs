//! Version 1 of the HTTP surface.
//!
//! One module per verb. A route is named after the curator verb it serves —
//! there is no second vocabulary to keep in step with the first, and no
//! `POST /call/{verb}` tunnel either: an RPC tunnel cannot be cached, cannot
//! be opened in a browser, and reads as noise in a log. A named GET does all
//! three.

pub mod at;
pub mod board;
pub mod chain;
pub mod export;
