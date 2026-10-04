use sqlx::PgPool;

pub async fn connect() -> PgPool {
    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/titeny".into());
    PgPool::connect(&url).await.expect("gagal konek DB titeny")
}
