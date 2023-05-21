use std::{error::Error, fs};

use std::path::{PathBuf, Path};

const DST_FOLDER: &str = "src/generated";

const PROTOS_DIR: &str = "ydb-api-protos";

fn main() -> Result<(), Box<dyn Error>> {
    return Ok(());
    if let Some(e) = fs::remove_dir_all(DST_FOLDER).err() {
        println!("{e}");
    };
    fs::create_dir(DST_FOLDER)?;
    let compile_files = make_files_list(PROTOS_DIR)?;

    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .out_dir(DST_FOLDER)
        .include_file("mod.rs")
        .compile_well_known_types(true)
        .compile(&compile_files, &[PROTOS_DIR])?;
    Ok(())
}

fn make_files_list(dir: impl AsRef<Path>) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let meta = entry.metadata()?;
        if meta.is_file() {
            if entry.path().to_str().unwrap().ends_with("_v1.proto") {
                let name = PathBuf::from(entry.file_name());
                files.push(name)
            }
            
        }
    }
    Ok(files)
}
