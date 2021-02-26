#[cfg(not(feature = "dynamic-loading"))]
#[cfg(all(target_os = "android", target_arch = "x86"))]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/generated/android/x86/bindings.rs"
));
#[cfg(not(feature = "dynamic-loading"))]
#[cfg(all(target_os = "android", target_arch = "x86_64"))]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/generated/android/x86_64/bindings.rs"
));
#[cfg(not(feature = "dynamic-loading"))]
#[cfg(all(target_os = "android", target_arch = "arm"))]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/generated/android/arm/bindings.rs"
));
#[cfg(not(feature = "dynamic-loading"))]
#[cfg(all(target_os = "android", target_arch = "aarch64"))]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/generated/android/aarch64/bindings.rs"
));

#[cfg(feature = "dynamic-loading")]
#[cfg(all(target_os = "android", target_arch = "x86"))]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/generated/android/x86/bindings_dynamic.rs"
));
#[cfg(feature = "dynamic-loading")]
#[cfg(all(target_os = "android", target_arch = "x86_64"))]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/generated/android/x86_64/bindings_dynamic.rs"
));
#[cfg(feature = "dynamic-loading")]
#[cfg(all(target_os = "android", target_arch = "arm"))]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/generated/android/arm/bindings_dynamic.rs"
));
#[cfg(feature = "dynamic-loading")]
#[cfg(all(target_os = "android", target_arch = "aarch64"))]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/generated/android/aarch64/bindings_dynamic.rs"
));
