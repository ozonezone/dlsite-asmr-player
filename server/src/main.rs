use anyhow::Result;
use axum::Router;
use std::sync::Arc;
use tokio::{signal, sync::RwLock};
use tracing::info;

use crate::{config::Config, prisma::user};
use prisma::PrismaClient;

mod axum_router;
mod config;
mod db;
#[allow(warnings, unused)]
mod prisma;
mod rspc_router;
mod scan;

type Db = Arc<PrismaClient>;

#[derive(Clone)]
pub(crate) struct AxumRouterState {
    pub db: Arc<PrismaClient>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .event_format(
            tracing_subscriber::fmt::format()
                .with_file(true)
                .with_line_number(true),
        )
        .init();

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

    let mut n = 0;
    let client = loop {
        let client = prisma::PrismaClient::_builder()
            .with_url(std::env::var("DATABASE_URL").expect("No DATABASE_URL environment variable"))
            .build()
            .await;
        n += 1;

        if let Ok(client) = client {
            break client;
        }

        info!(
            "Failed to connect database. Retrying in 5 seconds. (attempt {})",
            n
        );
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

        if n >= 10 {
            panic!("Failed to connect to database");
        }
    };

    #[cfg(debug_assertions)]
    {
        info!("db push");
        client._db_push().await.unwrap();
    }

    #[cfg(not(debug_assertions))]
    {
        info!("Running database migrations");
        client._migrate_deploy().await?;
        info!("Database migrations completed");
    }

    if client
        .user()
        .find_unique(user::id::equals(1))
        .exec()
        .await?
        .is_none()
    {
        info!("Creating default admin user with password 'password'");
        client
            .user()
            .create("admin".to_string(), "password".to_string(), vec![])
            .exec()
            .await?;
    }

    let client = Arc::new(client);

    let app = Router::new()
        .merge(rspc_router::mount(config.clone(), client.clone()))
        .merge(axum_router::mount(client.clone()))
        .with_state(AxumRouterState { db: client });

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
