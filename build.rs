use vergen::{Config, ShaKind};

fn main() {
    #[cfg(feature = "ffi")]
    {
        use std::env;

        use cbindgen::{Builder, Language};

        let output_file = format!(
            "{}/../../../headers/casper_client.h",
            env::var("OUT_DIR").expect("should have env var OUT_DIR set"),
        );

        let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        Builder::new()
            .with_crate(crate_dir)
            .with_language(Language::C)
            .with_autogen_warning(
                "/* WARNING: this file is autogenerated by cbindgen. Don't modify this manually. */"
            )
            .with_include_guard("__CASPER_CLIENT_H__")
            .with_no_includes()
            // add sys headers explicitly
            .with_sys_include("stdint.h")
            .with_sys_include("stdbool.h")
            // individual exported struct definitions need to be explicitly included
            .include_item("casper_deploy_params_t")
            .include_item("casper_payment_params_t")
            .include_item("casper_session_params_t")
            .generate()
            .expect("Unable to generate bindings")
            .write_to_file(output_file);
    }

    let mut config = Config::default();
    *config.git_mut().sha_kind_mut() = ShaKind::Short;
    let _ = vergen::vergen(config);
}
