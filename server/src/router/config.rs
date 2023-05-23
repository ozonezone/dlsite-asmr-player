use super::RouterBuilder;

pub(crate) fn mount() -> RouterBuilder {
    <RouterBuilder>::new()
        .mutation("setPassword", |t| {
            t(|ctx, new_password: String| async move {
                ctx.config.write().await.password = new_password;
                ctx.config.read().await.write_to_file().await.map_err(|e| {
                    rspc::Error::new(
                        rspc::ErrorCode::InternalServerError,
                        format!("Failed to write config to file: {}", e),
                    )
                })
            })
        })
        .mutation("setScandir", |t| {
            t(|ctx, dirs: Vec<String>| async move {
                ctx.config.write().await.scan_dir =
                    dirs.iter().map(std::path::PathBuf::from).collect();
                ctx.config.read().await.write_to_file().await.map_err(|e| {
                    rspc::Error::new(
                        rspc::ErrorCode::InternalServerError,
                        format!("Failed to write config to file: {}", e),
                    )
                })
            })
        })
        .query("getScandir", |t| {
            t(|ctx, _: ()| async move { ctx.config.read().await.scan_dir.clone() })
        })
}
