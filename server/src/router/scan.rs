use crate::scan;

use super::RouterBuilder;

pub(crate) fn mount() -> RouterBuilder {
    <RouterBuilder>::new().mutation("start", |t| {
        t(|ctx, _: ()| async move {
            if ctx.scan_status.read().await.is_scanning {
                return Err(rspc::Error::new(
                    rspc::ErrorCode::InternalServerError,
                    "Already scanning".into(),
                ));
            }

            scan::scan(&ctx.config.read().await.scan_dir, true, &ctx.pool)
                .await
                .map_err(|e| {
                    rspc::Error::new(
                        rspc::ErrorCode::InternalServerError,
                        format!("Scan failed: {}", e),
                    )
                })?;

            Ok(())
        })
    })
}
