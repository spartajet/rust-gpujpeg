fn main() {
    #[cfg(feature = "generate-bindings")]
    {
        let bindings = bindgen::Builder::default()
            .header("headers/gpujpeg.h")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            .generate()
            .expect("Unable to generate bindings");

        bindings
            .write_to_file("src/bindings.rs")
            .expect("Couldn't write bindings!");
    }
    // println!("cargo:rustc-link-search=libs/");
    // println!("cargo:rustc-link-lib=gpujpeg");
    let install_dir = cmake::Config::new("GPUJPEG")
        .define("DCMAKE_BUILD_TYPE", "Release")
        .define("DCMAKE_CUDA_ARCHITECTURES", "native")
        .define("DBUILD_SHARED_LIBS", "FALSE")
        .build();
    println!("install_dir:{}", install_dir.display());
    println!(
        "cargo:rustc-link-search=native={}/lib",
        install_dir.display()
    );
    println!("cargo:rustc-link-lib=static=gpujpeg");
}
