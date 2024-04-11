use std::env;
use std::path::{Path, PathBuf};

use coin_build_tools::{coinbuilder, link, utils};

const LIB_NAME: &str = "Clp";

fn main() {
    println!(
        "cargo:rerun-if-changed={}_lib_sources.txt",
        LIB_NAME.to_ascii_lowercase()
    );
    println!(
        "cargo:rerun-if-env-changed=CARGO_{}_STATIC",
        LIB_NAME.to_ascii_uppercase()
    );
    println!(
        "cargo:rerun-if-env-changed=CARGO_{}_SYSTEM",
        LIB_NAME.to_ascii_uppercase()
    );

    let want_system = utils::want_system(LIB_NAME);

    if want_system && link::link_lib_system_if_supported(LIB_NAME) {
        let mut coinflags = vec!["CLP".to_string()];

        let link_type = if utils::want_static(LIB_NAME) {
            "static".to_string()
        } else {
            "dylib".to_string()
        };

        if cfg!(feature = "osiclp") {
            println!("cargo:rustc-link-lib={}=OsiClp", link_type);
            coinflags.push("OSICLP".to_string());

            let (_, coinflags_other) = coinbuilder::get_metadata_from("Osi");
            coinflags.extend(coinflags_other);
        }

        if cfg!(feature = "clpsolver") {
            println!("cargo:rustc-link-lib={}=ClpSolver", link_type);
            coinflags.push("CLPSOLVER".to_string());
        }

        let (_, coinflags_other) = coinbuilder::get_metadata_from("CoinUtils");
        coinflags.extend(coinflags_other);

        coinbuilder::print_metadata(Vec::new(), coinflags);
        return;
    }

    if !Path::new(&format!("{}/LICENSE", LIB_NAME)).exists() {
        utils::update_submodules(env::var("CARGO_MANIFEST_DIR").unwrap());
    }
    build_lib_and_link();
}

fn build_lib_and_link() {
    let src_dir = format!(
        "{}",
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
            .join(LIB_NAME)
            .join("Clp")
            .join("src")
            .display()
    );

    let mut includes_dir = vec![format!("{}", src_dir)];

    let mut lib_sources = include_str!("clp_lib_sources.txt")
        .trim()
        .split('\n')
        .map(|file| format!("{}/{}", src_dir, file.trim()))
        .collect::<Vec<String>>();

    let mut coinflags = vec!["CLP".to_string()];

    if cfg!(feature = "osiclp") {
        lib_sources.push(format!("{}/OsiClp/OsiClpSolverInterface.cpp", src_dir));
        includes_dir.push(format!("{}/OsiClp", src_dir));
        coinflags.push("OSICLP".to_string());
    }

    let (include_other, coinflags_other) = coinbuilder::get_metadata_from("CoinUtils");
    includes_dir.extend(include_other);
    coinflags.extend(coinflags_other);

    if cfg!(feature = "osiclp") {
        let (include_other, coinflags_other) = coinbuilder::get_metadata_from("Osi");
        includes_dir.extend(include_other);
        coinflags.extend(coinflags_other);
    }

    coinbuilder::print_metadata(includes_dir.clone(), coinflags.clone());

    let mut config = coinbuilder::init_builder();
    coinflags.iter().for_each(|flag| {
        config.define(&format!("COIN_HAS_{}", flag), None);
    });
    config.includes(includes_dir.clone());
    config.files(lib_sources);

    config.compile("Clp");

    if cfg!(feature = "clpsolver") {
        let lib_sources = include_str!("clpsolver_lib_sources.txt")
            .trim()
            .split('\n')
            .map(|file| format!("{}/{}", src_dir, file.trim()))
            .collect::<Vec<String>>();

        let mut config = coinbuilder::init_builder();
        coinflags.iter().for_each(|flag| {
            config.define(&format!("COIN_HAS_{}", flag), None);
        });
        config.includes(includes_dir);
        config.files(lib_sources);

        config.compile("ClpSolver");
    }
}
