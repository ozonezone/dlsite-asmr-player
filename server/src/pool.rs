use deadpool_postgres::{Config, CreatePoolError, Pool, Runtime};
use tokio_postgres::NoTls;

#[cfg(debug_assertions)]
use load_dotenv::load_dotenv;
#[cfg(debug_assertions)]
load_dotenv!();

macro_rules! env {
    ($lit: literal) => {
        if let Some(value) = option_env!($lit) {
            value.to_string()
        } else {
            std::env::var($lit).unwrap().to_string()
        }
    };
}

pub(crate) async fn create_pool() -> Result<Pool, CreatePoolError> {
    let mut cfg = Config::new();
    cfg.user = Some(env!("PG_USER"));
    cfg.password = Some(env!("PG_PASSWORD"));
    cfg.host = Some(env!("PG_HOST"));
    cfg.port = Some(env!("PG_PORT").parse().unwrap());
    cfg.dbname = Some(env!("PG_DBNAME"));
    cfg.create_pool(Some(Runtime::Tokio1), NoTls)
}
