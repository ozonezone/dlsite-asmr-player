use rspc::Type;
use serde::Deserialize;

use crate::{config::Config, db::config::change_pass};

use super::{utils::ToRspcInternalError, RouterBuilder};

#[derive(Deserialize, Type)]
struct NewPasswordArgs {
    password: String,
    new_password: String,
}

pub(crate) fn mount() -> RouterBuilder {
    <RouterBuilder>::new()
        .mutation("setPassword", |t| {
            t(|ctx, args: NewPasswordArgs| async move {
                change_pass(ctx.db, ctx.user_id, &args.password, args.new_password)
                    .await
                    .to_rspc_internal_error("Error")
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
