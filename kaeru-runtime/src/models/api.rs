//! Resolved authorization supplied by an authenticated host boundary.

/// One exact initiative and an opaque caller identity, already authorized by
/// the host. Never place a raw credential in `caller`. The server additionally
/// intersects this grant with its operator-owned scope and visibility ceiling.
pub struct AuthorizedRead {
    pub initiative: String,
    pub caller: String,
}
