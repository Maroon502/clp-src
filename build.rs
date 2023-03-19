use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

use pkg_config;

const LIB_NAME: &str = "Clp";

fn main() {
    println!("cargo:rerun-if-changed={}_lib_sources.txt", LIB_NAME.to_ascii_lowercase());
    println!("cargo:rerun-if-changed={}_STATIC", LIB_NAME.to_ascii_uppercase());
    
    if cfg!(feature = "system") {
        match link_lib_system(LIB_NAME) {
            true => return,
            false => (),
        }
    }

    if !Path::new(&format!("{}/LICENSE", LIB_NAME)).exists() {
        update_submodules();
    }
    build_lib_and_link();

}

fn update_submodules() {
    let program = "git";
    let dir = "../";
    let args = ["submodule", "update", "--init"];
    println!(
        "Running command: \"{} {}\" in dir: {}",
        program,
        args.join(" "),
        dir
    );
    let ret = Command::new(program).current_dir(dir).args(args).status();

    match ret.map(|status| (status.success(), status.code())) {
        Ok((true, _)) => (),
        Ok((false, Some(c))) => panic!("Command failed with error code {}", c),
        Ok((false, None)) => panic!("Command got killed"),
        Err(e) => panic!("Command failed with error: {}", e),
    }
}

fn build_lib_and_link() {
    let target = env::var("TARGET").unwrap();

    let mut config = cc::Build::new()
        .cpp(true)
        .warnings(false)
        .extra_warnings(false)
        .define("NDEBUG", None)
        .define("HAVE_STDIO_H", None)
        .define("HAVE_STDLIB_H", None)
        .define("HAVE_STRING_H", None)
        .define("HAVE_INTTYPES_H", None)
        .define("HAVE_STDINT_H", None)
        .define("HAVE_STRINGS_H", None)
        .define("HAVE_SYS_TYPES_H", None)
        .define("HAVE_SYS_STAT_H", None)
        .define("HAVE_UNISTD_H", None)
        .define("HAVE_CMATH", None)
        .define("HAVE_CFLOAT", None)
        .define("HAVE_DLFCN_H", None)
        .define("HAVE_MEMORY_H", None)
        .to_owned();

    if target.contains("msvc") {
        config.flag("-EHsc")
        .flag_if_supported("-std:c++11");
    } else {
        config.flag("-std=c++11").flag("-w");
    }

    let src_dir = format!(
        "{}",
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
            .join(LIB_NAME)
            .join("Clp")
            .join("src")
            .display()
    );

    let mut includes_dir = vec![
        format!("{}", src_dir),
    ];

    let mut lib_sources = include_str!("clp_lib_sources.txt")
        .trim()
        .split('\n')
        .map(|file| format!("{}/{}", src_dir, file.trim()))
        .collect::<Vec<String>>();

    if cfg!(feature = "with_osi") {
        lib_sources.push(format!("{}/OsiClp/OsiClpSolverInterface.cpp", src_dir));
        includes_dir.push(format!("{}/OsiClp", src_dir));
        if let Some(paths) = env::var_os("DEP_OSI_INCLUDE") {
            env::split_paths(&paths).for_each(|p| {
                includes_dir.push(format!("{}", p.display()));
            });
        }
    }

    includes_dir.iter().for_each(|p| println!("cargo:include={}", p));
    
    if let Some(paths) = env::var_os("DEP_COINUTILS_INCLUDE") {
        env::split_paths(&paths).for_each(|p| {
            includes_dir.push(format!("{}", p.display()));
        });
    }

    config.includes(includes_dir);
    config.files(lib_sources);

    config.compile("Clp");
}

fn link_lib_system(lib_name: &str) -> bool {
    let host = env::var("HOST").unwrap();
    let target = env::var("TARGET").unwrap();
    let host_and_target_contain = |s| host.contains(s) && target.contains(s);

    if target.contains("msvc") {
        link_windows_msvc_system(lib_name)
    } else if !(host_and_target_contain("apple") ||
        host_and_target_contain("freebsd") ||
        host_and_target_contain("dragonfly"))
    {
        link_linux_gnu_system(lib_name)
    } else {
        false
    }
}

fn link_linux_gnu_system(lib_name: &str) -> bool{
    let want_static = cfg!(feature = "static") || env::var_os(format!("{}_STATIC", lib_name.to_ascii_uppercase())).is_some();
    let mut cfg = pkg_config::Config::new();
    cfg.cargo_metadata(true)
        .print_system_libs(false);

    if want_static{
        cfg.statik(true);
    }

    match cfg.probe(lib_name) {
        Ok(lib) => {
            for include in lib.include_paths {
                println!("cargo:include={}", include.display());
            }
            true
        }
        Err(e) => {
            println!("pkg-config did not find {}: {}", lib_name, e);
            false
        }
    }
}

fn link_windows_msvc_system(lib_name: &str) -> bool{
    let want_static = cfg!(feature = "static") || env::var_os(format!("{}_STATIC", lib_name.to_ascii_uppercase())).is_some();
    if !want_static {
        env::set_var("VCPKGRS_DYNAMIC", "1");
    }

    match vcpkg::Config::new()
        .emit_includes(true)
        .lib_name(lib_name)
        .probe(lib_name)
    {
        Ok(_) => {
            if want_static {
                println!("cargo:rustc-link-lib=static={}", lib_name);
            } else {
                println!("cargo:rustc-link-lib={}", lib_name);
            }
            true
        }
        Err(e) => {
            println!("vcpkg did not find {}: {}", lib_name, e);
            false
        }
    }
}
