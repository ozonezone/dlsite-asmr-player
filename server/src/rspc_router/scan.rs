use crate::scan;

use super::RouterBuilder;

pub(crate) fn mount() -> RouterBuilder {
    <RouterBuilder>::new().mutation("start", |t| {
        t(|ctx, force: bool| async move {
            if ctx.scan_status.read().await.is_scanning {
                return Err(rspc::Error::new(
                    rspc::ErrorCode::InternalServerError,
                    "Already scanning".into(),
                ));
            }

            let deleted_count = scan::scan(&ctx.config.read().await.scan_dir, force, ctx.db)
                .await
                .map_err(|e| {
                    rspc::Error::new(
                        rspc::ErrorCode::InternalServerError,
                        format!("Scan failed: {}", e),
                    )
                })?;

            Ok(deleted_count as i32)
        })
    })
}
