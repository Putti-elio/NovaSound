#![allow(
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::unwrap_used,
    clippy::uninlined_format_args,
    clippy::redundant_closure_for_method_calls,
    clippy::single_char_pattern,
    clippy::needless_pass_by_value,
    clippy::must_use_candidate
)]

pub mod database;
pub mod errors;
pub mod migrations;
pub mod models;
pub mod rpc;
pub mod services;
pub mod utils;

#[cfg(test)]
mod tests {
    mod album;
    mod artist;
    mod connect;
    mod song;
    pub mod test_helpers;
}
