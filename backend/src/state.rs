use deadpool_postgres::Pool;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Pool,
}

impl AppState {
    #[must_use]
    pub fn new(db_pool: Pool) -> Self {
        Self { db_pool }
    }
}
