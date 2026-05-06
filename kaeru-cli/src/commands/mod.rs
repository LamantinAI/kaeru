//! Subcommand handlers, grouped by curator-API domain. Each file
//! exposes one `pub fn` per subcommand variant declared in `main.rs`;
//! the dispatch in `main.rs` is one `Command::X => commands::group::x(...)?`
//! per arm.

pub mod capture;
pub mod consolidate;
pub mod hypothesis;
pub mod lint;
pub mod lookup;
pub mod metabolism;
pub mod review;
pub mod session;
pub mod temporal;
pub mod vault;
