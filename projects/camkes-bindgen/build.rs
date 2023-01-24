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
    let include_dirs: Vec<PathBuf> = get_include_dirs(&build_dir)?.collect();

    let mut includes = Vec::new();

    for (name, camkes_header) in camkes_headers {
        generate_bindings(&out_dir, &name, &camkes_header, include_dirs.iter())?;

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

fn get_camkes_headers(
    build_dir: impl AsRef<Path>,
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

fn get_include_dirs(build_dir: impl AsRef<Path>) -> Result<impl Iterator<Item = PathBuf>> {
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
        .unique()
        .map(move |dir| build_dir.join(dir)))
}

fn generate_bindings(
    out_dir: impl AsRef<Path>,
    name: impl AsRef<str>,
    camkes_header: impl AsRef<Path>,
    include_dirs: impl Iterator<Item = impl AsRef<Path>>,
) -> Result<()> {
    let bindings = Builder::default()
        .header(camkes_header.as_ref().to_str().unwrap())
        .clang_args(
            include_dirs
                .into_iter()
                .map(|dir| format!("-I{}", dir.as_ref().to_str().unwrap())),
        )
        .use_core()
        .layout_tests(false)
        .generate()?;

    bindings.write_to_file(out_dir.as_ref().join(format!("{}.rs", name.as_ref())))?;

    Ok(())
}
