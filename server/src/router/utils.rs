pub(crate) trait ToRspcError<T> {
    fn to_rspc_internal_error(self, msg: &str) -> Result<T, rspc::Error>;
}

impl<T, E: std::fmt::Display> ToRspcError<T> for Result<T, E> {
    fn to_rspc_internal_error(self, msg: &str) -> Result<T, rspc::Error> {
        self.map_err(|e| {
            rspc::Error::new(
                rspc::ErrorCode::InternalServerError,
                format!("{}: {}", msg, e),
            )
        })
    }
}
