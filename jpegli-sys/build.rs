use std::{
    env,
    path::{Path, PathBuf},
};

fn source_dir() -> PathBuf {
    env::var("DEP_JXL_PATH").map_or_else(
        |_| Path::new(env!("CARGO_MANIFEST_DIR")).join("libjxl"),
        PathBuf::from,
    )
}

pub fn main() {
    let source = source_dir();

    if let Ok(p) = std::thread::available_parallelism() {
        env::set_var("CMAKE_BUILD_PARALLEL_LEVEL", format!("{}", p))
    }

    let mut config = cmake::Config::new(source);
    config
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("BUILD_TESTING", "OFF")
        .define("JPEGXL_ENABLE_JPEGLI", "ON")
        .define("JPEGXL_ENABLE_TOOLS", "OFF")
        .define("JPEGXL_ENABLE_DOXYGEN", "OFF")
        .define("JPEGXL_ENABLE_MANPAGES", "OFF")
        .define("JPEGXL_ENABLE_BENCHMARK", "OFF")
        .define("JPEGXL_ENABLE_EXAMPLES", "OFF")
        .define("JPEGXL_ENABLE_JNI", "OFF")
        .define("JPEGXL_ENABLE_SJPEG", "OFF")
        .define("JPEGXL_ENABLE_OPENEXR", "OFF")
        .define("JPEGLI_LIBJPEG_LIBRARY_SOVERSION", "8")
        .define("JPEGLI_LIBJPEG_LIBRARY_VERSION", "8.2.2")
        .build_target("jpegli-static");

    let build_dir = config.build();

    // On Windows MSVC, CMake outputs to lib/Release/ subdirectory
    // On Unix, it outputs directly to lib/
    let lib_path = build_dir.join("build").join("lib");
    println!("cargo:rustc-link-search=native={}", lib_path.display());
    #[cfg(target_env = "msvc")]
    println!("cargo:rustc-link-search=native={}", lib_path.join("Release").display());

    let highway_path = build_dir.join("build").join("third_party").join("highway");
    println!("cargo:rustc-link-search=native={}", highway_path.display());
    #[cfg(target_env = "msvc")]
    println!("cargo:rustc-link-search=native={}", highway_path.join("Release").display());

    println!("cargo:rustc-link-lib=jpegli-static");
    println!("cargo:rustc-link-lib=hwy");

    #[cfg(any(target_os = "macos", target_os = "ios", target_os = "freebsd"))]
    println!("cargo:rustc-link-lib=c++");
    #[cfg(not(any(
        target_os = "macos",
        target_os = "ios",
        target_os = "freebsd",
        target_env = "msvc"
    )))]
    println!("cargo:rustc-link-lib=stdc++");
}
