pub mod database;
pub mod errors;
pub mod models;
pub mod routes;
pub mod services;
pub mod utils;

#[cfg(test)]
mod tests {
    mod album;
    mod artist;
    mod song;
}