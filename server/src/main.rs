use anyhow::Result;
use axum::Router;
use entity::entities::user;
use migration::{sea_orm::Database, Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, DatabaseConnection, EntityTrait, Set};
use std::sync::Arc;
use tokio::{signal, sync::RwLock};
use tracing::info;

use crate::config::Config;

mod axum_router;
mod config;
mod db;
mod rspc_router;
mod scan;

#[derive(Clone)]
pub(crate) struct AxumRouterState {
    pub db: DatabaseConnection,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    let config = match Config::from_file().await {
        Ok(config) => config,
        Err(e) => {
            info!("Failed to load config file: {:?}", e);
            info!("Creating default config file");
            let config = Config::default();
            let config_path = config.write_to_file().await?;
            let config_path = config_path.to_str().unwrap_or("");
            info!("Config file created to: {}", config_path);
            config
        }
    };

    let config = Arc::new(RwLock::new(config));

    let mut opt = ConnectOptions::new(std::env::var("DATABASE_URL").unwrap());
    opt.sqlx_logging_level(tracing::log::LevelFilter::Debug);
    let db = Database::connect(opt).await?;

    info!("Running database migrations");
    Migrator::up(&db, None).await?;
    info!("Database migrations completed");

    if user::Entity::find().one(&db).await?.is_none() {
        info!("Creating default admin user with password 'password'");
        user::Entity::insert(user::ActiveModel {
            id: Set(1),
            password: Set("password".to_string()),
            name: Set("admin".to_string()),
        })
        .exec(&db)
        .await?;
    }

    let app = Router::new()
        .merge(rspc_router::mount(config.clone(), db.clone()))
        .merge(axum_router::mount(db.clone()))
        .with_state(AxumRouterState { db });

    let addr = "[::]:14567".parse::<std::net::SocketAddr>().unwrap();
    println!("listening on http://{}/rspc", addr);

    let axum_task = axum::Server::bind(&addr).serve(app.into_make_service());

    tokio::select! {
        _ = signal::ctrl_c() => {
                info!("Ctrl-C received, shutting down");
        },
        _ = axum_task => {},
    }

    Ok(())
}
