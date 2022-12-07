pub(crate) fn log_request(req: &worker::Request) {
    worker::console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        worker::Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or_else(|| "unknown region".into())
    );
}
