use crate::{config::Config, cornucopia::queries::user::change_password};

use super::{utils::ToRspcError, RouterBuilder};

pub(crate) fn mount() -> RouterBuilder {
    <RouterBuilder>::new()
        .mutation("setPassword", |t| {
            t(|ctx, new_password: String| async move {
                let client = ctx
                    .pool
                    .get()
                    .await
                    .to_rspc_internal_error("Cannot connect db")?;
                change_password()
                    .bind(&client, &new_password, &1)
                    .await
                    .to_rspc_internal_error("Could not update password")?;

                Ok(())
            })
        })
        .mutation("setConfig", |t| {
            t(|ctx, config: Config| async move {
                *ctx.config.write().await = config;
                ctx.config
                    .read()
                    .await
                    .write_to_file()
                    .await
                    .to_rspc_internal_error("Failed to write config to file")
            })
        })
        .query("getConfig", |t| {
            t(|ctx, _: ()| async move { ctx.config.read().await.clone() })
        })
}
