use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use anyhow::Result;
use bindgen::Builder;
use glob::glob;
use itertools::Itertools;

fn main() -> Result<()> {
    let build_dir: PathBuf = env::var("RUST_BUILD_DIR")?.into();
    let out_dir: PathBuf = env::var("OUT_DIR")?.into();

    let camkes_headers = get_camkes_headers(&build_dir)?;
    let mut includes = Vec::new();
    for (name, camkes_header) in camkes_headers {
        generate_bindings(&build_dir, &out_dir, &name, &camkes_header)?;

        includes.push(format!("pub mod {name} {{ include!(\"{name}.rs\"); }}"));
        println!(
            "cargo:rerun-if-changed={}",
            camkes_header
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .to_str()
                .unwrap()
        );
    }

    std::fs::write(out_dir.join("bindings.rs"), includes.join("\n"))?;

    println!(
        "cargo:rerun-if-changed={}",
        build_dir.join("build.ninja").to_str().unwrap()
    );

    Ok(())
}

fn get_camkes_headers<P: AsRef<Path>>(
    build_dir: P,
) -> Result<impl Iterator<Item = (String, PathBuf)>> {
    let build_dir = build_dir.as_ref().to_path_buf();
    let camkes_pathes = glob(build_dir.join("*/include/camkes.h").to_str().unwrap())?;

    Ok(camkes_pathes.filter_map(|path| path.ok()).map(|path| {
        let name = path
            .components()
            .nth_back(2)
            .unwrap()
            .as_os_str()
            .to_str()
            .unwrap()
            .to_string();

        (name, path)
    }))
}

fn get_include_dirs<P: AsRef<Path>>(build_dir: P) -> Result<impl Iterator<Item = PathBuf>> {
    let build_dir = build_dir.as_ref().to_path_buf();

    let build_file_path = build_dir.join("build.ninja");
    let buffer = BufReader::new(File::open(build_file_path)?);

    let get_includes_line = |line: String| line.strip_prefix("  INCLUDES = ").map(str::to_string);
    let get_include_dirs = |line: String| {
        line.split(' ')
            .filter_map(|stmt| stmt.strip_prefix("-I"))
            .map(PathBuf::from)
            .collect::<Vec<PathBuf>>()
    };

    Ok(buffer
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(get_includes_line)
        .flat_map(get_include_dirs)
        //.flatten()
        .unique()
        .map(move |dir| build_dir.join(dir)))
}

fn generate_bindings<P: AsRef<Path>>(
    build_dir: P,
    out_dir: P,
    name: &str,
    camkes_header: P,
) -> Result<()> {
    let include_dirs = get_include_dirs(&build_dir)?;

    let bindings = Builder::default()
        .header(camkes_header.as_ref().to_str().unwrap())
        .clang_args(include_dirs.map(|dir| format!("-I{}", dir.to_str().unwrap())))
        .use_core()
        .layout_tests(false)
        .generate()?;

    bindings.write_to_file(out_dir.as_ref().join(format!("{name}.rs")))?;

    Ok(())
}
