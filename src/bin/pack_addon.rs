use clap::Parser;
use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use zip::ZipWriter;
use zip_extensions::zip_writer_extensions::ZipWriterExtensions;

const PROJECT_NAME: &str = "cybuild_godot";

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(long)]
    release: bool,
}

#[derive(Debug, Eq, PartialEq, Hash)]
enum Target {
    Windows,
    Linux,
    MacOS,
}

fn main() {
    let cli = Cli::parse();

    let mappings = HashMap::from([(
        Target::Windows,
        "target/x86_64-pc-windows-msvc/$BuildMode$/$ProjectName$.dll",
    )]);

    // Check paths
    let current_path = env::current_dir().unwrap();
    let target_path = current_path.join("target");
    let pack_path = current_path.join("pack");

    if !target_path.is_dir() {
        panic!("Not in project directory or haven't yet built");
    }

    if !pack_path.is_dir() {
        fs::create_dir(&pack_path).expect("Failed to create directory");
    }

    // Copy artifacts
    for i in mappings {
        let dll_path = PathBuf::from(replace_vars(i.1, &cli));
        let dll_folder = get_artifact_folder(i.0, &pack_path);
        let mut dll_dest = PathBuf::from(&dll_folder);
        dll_dest.push(dll_path.file_name().unwrap());

        fs::create_dir_all(&dll_folder).expect("Failed to create directory");

        println!("Copy: {} -> {}", dll_path.display(), dll_dest.display());
        fs::copy(dll_path, dll_dest).unwrap();
    }

    // Write .gdextension file
    let gdx = pack_path.join("cybuild.gdextension");
    write_gedextension(gdx.as_path()).unwrap();

    // Create archive file
    println!(
        "Create archive: {} -> {}",
        pack_path.display(),
        "cybuild.zip"
    );
    let file = File::create("cybuild.zip").unwrap();
    let mut zip = ZipWriter::new(file);
    zip.create_from_directory(&pack_path).unwrap();
}

fn replace_vars(original: &str, cli: &Cli) -> String {
    original
        .replace("$BuildMode$", if cli.release { "release" } else { "debug" })
        .replace("$ProjectName$", PROJECT_NAME)
}

fn get_artifact_folder(target: Target, pack_path: &PathBuf) -> String {
    match target {
        Target::Windows => pack_path.join("lib/windows/").to_str().unwrap().to_owned(),
        _ => panic!("Failed to get destination"),
    }
}

fn write_gedextension(path: &Path) -> anyhow::Result<()> {
    let mut f = File::open("cybuild.gdextension")?;
    let mut gdx_str = String::new();
    f.read_to_string(&mut gdx_str)?;
    gdx_str = gdx_str.replace("{project_name}", PROJECT_NAME);
    let mut gdx = File::create(path)?;
    gdx.write_all(gdx_str.as_bytes())?;

    // TODO
    Ok(())
}
