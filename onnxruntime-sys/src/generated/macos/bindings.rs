#[cfg(not(feature = "dynamic-loading"))]
#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
include!("./x86_64/bindings.rs");

#[cfg(feature = "dynamic-loading")]
#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
include!("./x86_64/bindings_dynamic.rs");
