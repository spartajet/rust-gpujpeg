fn main() {
    println!("cargo:rustc-link-search=libs/");
    println!("cargo:rustc-link-lib=gpujpeg");

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
}
