use entity::entities::user;
use rspc::Type;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::Deserialize;

use crate::config::Config;

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
                let user = user::Entity::find_by_id(1)
                    .one(&ctx.db)
                    .await
                    .to_rspc_internal_error("Db connect error")?
                    .to_rspc_internal_error("User not found")?;
                if args.password == user.password {
                    let mut user: user::ActiveModel = user.into();
                    user.password = Set(args.new_password.clone());
                    user.update(&ctx.db)
                        .await
                        .to_rspc_internal_error("Could not update password")?;

                    Ok(())
                } else {
                    Err(rspc::Error::new(
                        rspc::ErrorCode::Unauthorized,
                        "Wrong password".into(),
                    ))
                }
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
