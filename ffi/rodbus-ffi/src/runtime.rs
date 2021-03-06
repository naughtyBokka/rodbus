pub use tokio::runtime::Runtime;

pub(crate) unsafe fn runtime_new(
    config: Option<&crate::ffi::RuntimeConfig>,
) -> *mut tokio::runtime::Runtime {
    let mut builder = tokio::runtime::Builder::new();

    builder.enable_all().threaded_scheduler();

    if let Some(x) = config.as_ref() {
        if x.num_core_threads > 0 {
            builder.core_threads(x.num_core_threads as usize);
        }
    }

    match builder.build() {
        Ok(r) => Box::into_raw(Box::new(r)),
        Err(_) => std::ptr::null_mut(),
    }
}

pub(crate) unsafe fn runtime_destroy(runtime: *mut crate::Runtime) {
    if !runtime.is_null() {
        Box::from_raw(runtime);
    };
}
