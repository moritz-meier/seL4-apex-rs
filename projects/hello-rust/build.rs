use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use anyhow::Result;

use bindgen::Builder;
use itertools::Itertools;

fn main() -> Result<()> {
    let include_dirs = get_include_dirs("/workspaces/seL4-apex-rs/build")?;

    let bindings = Builder::default()
        .header("/workspaces/seL4-apex-rs/build/main_obj/include/camkes.h")
        .clang_args(include_dirs.map(|dir| format!("-I{}", dir.as_os_str().to_str().unwrap())))
        .use_core()
        .layout_tests(false)
        .generate()?;

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))?;

    Ok(())
}

fn get_include_dirs<P: AsRef<Path>>(build_dir: P) -> Result<impl Iterator<Item = PathBuf>> {
    let build_dir = build_dir.as_ref().to_path_buf();

    let build_file_path = build_dir.join("build.ninja");
    let buffer = BufReader::new(File::open(build_file_path)?);

    let get_includes_line = |line: String| line.strip_prefix("  INCLUDES = ").map(str::to_string);
    let get_include_dirs = |line: String| {
        line.split(" ")
            .filter_map(|stmt| stmt.strip_prefix("-I"))
            .map(PathBuf::from)
            .collect::<Vec<PathBuf>>()
    };

    Ok(buffer
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(get_includes_line)
        .map(get_include_dirs)
        .flatten()
        .unique()
        .map(move |dir| build_dir.join(dir)))
}
