use std::env;

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=dlib");

    // ---------------------------------------------------------
    // Configure dlib, the machine learning toolkit
    // ---------------------------------------------------------
    let mut config = cmake::Config::new("dlib");
    configure_cmake_tools(&mut config);
    // prefer static libraries for Rust
    config.define("BUILD_SHARED_LIBS", "OFF");
    // disable testing
    config.define("BUILD_TESTING", "OFF");
    config.define("DLIB_NO_GUI_SUPPORT_STR", "ON");
    config.define("DLIB_USE_BLAS_STR", "OFF");
    config.define("DLIB_USE_LAPACK_STR", "OFF");
    config.define("DLIB_USE_CUDA_STR", "OFF");
    config.define("DLIB_PNG_SUPPORT_STR", "OFF");
    config.define("DLIB_GIF_SUPPORT_STR", "OFF");
    config.define("DLIB_JPEG_SUPPORT_STR", "OFF");
    config.define("DLIB_WEBP_SUPPORT_STR", "OFF");
    config.define("DLIB_JXL_SUPPORT_STR", "OFF");
    config.define("DLIB_LINK_WITH_SQLITE3_STR", "OFF");
    config.define("DLIB_USE_MKL_FFT_STR", "OFF");
    config.define("DLIB_USE_FFMPEG_STR", "OFF");
    let dlib_out = config.build();

    println!("cargo::rustc-link-search=native={}", dlib_out.display());
    println!(
        "cargo::rustc-link-search=native={}",
        dlib_out.join("lib").display()
    );
    println!(
        "cargo::rustc-link-search=native={}",
        dlib_out.join("lib64").display()
    );
    println!("cargo::rustc-link-lib=static=dlib");

    println!("cargo::metadata=root={}", dlib_out.display());
    println!(
        "cargo::metadata=include={}",
        dlib_out.join("include").display()
    );
}

fn configure_cmake_tools(config: &mut cmake::Config) {
    if let Ok(ar) = env::var("AR") {
        config.define("CMAKE_AR", ar);
    }
    if let Ok(ld) = env::var("LD") {
        config.define("CMAKE_LINKER", ld);
    }
    if let Ok(nm) = env::var("NM") {
        config.define("CMAKE_NM", nm);
    }
    if let Ok(objdump) = env::var("OBJDUMP") {
        config.define("CMAKE_OBJDUMP", objdump);
    }
    if let Ok(ranlib) = env::var("RANLIB") {
        config.define("CMAKE_RANLIB", ranlib);
    }
    if let Ok(strip) = env::var("STRIP") {
        config.define("CMAKE_STRIP", strip);
    }
}
