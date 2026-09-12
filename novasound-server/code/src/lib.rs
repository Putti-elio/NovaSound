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

pub mod errors;
pub mod migrations;
pub use novasound_domain::models;
pub mod adapters;
pub mod rpc;
pub mod state;
pub mod utils;
pub mod web;

pub use novasound_application::create_error;

#[cfg(test)]
mod tests {
    mod album;
    mod artist;
    mod connect;
    mod song;
    pub mod test_helpers;
}
