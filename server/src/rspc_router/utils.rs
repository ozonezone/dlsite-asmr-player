pub(crate) trait ToRspcInternalError<T> {
    fn to_rspc_internal_error(self, msg: &str) -> Result<T, rspc::Error>;
}

impl<T, E: std::fmt::Display> ToRspcInternalError<T> for Result<T, E> {
    fn to_rspc_internal_error(self, msg: &str) -> Result<T, rspc::Error> {
        self.map_err(|e| {
            rspc::Error::new(
                rspc::ErrorCode::InternalServerError,
                format!("{}: {}", msg, e),
            )
        })
    }
}

pub(crate) trait ToRspcNotFound<T> {
    fn to_rspc_not_found(self, msg: &str) -> Result<T, rspc::Error>;
}

impl<T> ToRspcNotFound<T> for Option<T> {
    fn to_rspc_not_found(self, msg: &str) -> Result<T, rspc::Error> {
        self.ok_or_else(|| rspc::Error::new(rspc::ErrorCode::NotFound, msg.to_string()))
    }
}
impl<T> ToRspcInternalError<T> for Option<T> {
fn to_rspc_internal_error(self, msg: &str) -> Result<T, rspc::Error> {
    self.ok_or_else(|| {
        rspc::Error::new(rspc::ErrorCode::InternalServerError, msg.to_string())
    })
}

}
