use entity::entities::user;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::config::Config;

use super::{utils::ToRspcError, RouterBuilder};

pub(crate) fn mount() -> RouterBuilder {
    <RouterBuilder>::new()
        .mutation("setPassword", |t| {
            t(|ctx, new_password: String| async move {
                user::Entity::update(user::ActiveModel {
                    password: sea_orm::ActiveValue::Set(new_password.clone()),
                    ..Default::default()
                })
                .filter(user::Column::Id.eq(1))
                .exec(&ctx.pool)
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
