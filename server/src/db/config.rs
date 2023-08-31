use anyhow::{Context, Result};

use crate::{prisma::user, Db};

pub async fn change_pass(
    db: Db,
    user_id: i32,
    old_pass: &str,
    new_pass: String,
) -> Result<user::Data> {
    let user = db
        .user()
        .find_unique(user::id::equals(user_id))
        .exec()
        .await
        .context("Db connect error")?
        .context("User not found")?;

    if old_pass == user.password {
        let data = db
            .user()
            .update(
                user::id::equals(user_id),
                vec![user::password::set(new_pass)],
            )
            .exec()
            .await?;
        Ok(data)
    } else {
        Err(anyhow::anyhow!("Wrong password"))
    }
}
