#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/generated/linux/x86_64/bindings.rs"
));

#[cfg(all(target_os = "macos"))]
include!("./macos/bindings.rs");

#[cfg(all(target_os = "windows", target_arch = "x86"))]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/generated/windows/x86/bindings.rs"
));

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/generated/windows/x86_64/bindings.rs"
));

#[cfg(target_os = "android")]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/generated/android/bindings.rs"
));
