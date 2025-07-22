use sqlx::{Pool, Postgres};
 
pub struct DbPool {
    pub pool: Pool<Postgres>,
} 