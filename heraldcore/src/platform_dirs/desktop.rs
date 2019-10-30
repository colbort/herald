use lazy_static::*;
use std::path::PathBuf;

lazy_static! {
    pub(super) static ref DATA_DIR: PathBuf = data_dir();
}

#[cfg(all(not(test), feature = "deploy_desktop"))]
fn data_dir() -> PathBuf {
    use directories::ProjectDirs;
    use std::fs::create_dir_all;

    let project_dir = match ProjectDirs::from("io", "Kalix Systems", "Herald") {
        Some(proj_dir) => proj_dir,
        None => match ProjectDirs::from_path("".into()) {
            Some(dir) => dir,
            None => return "".into(),
        },
    };

    let data_dir = project_dir.data_local_dir();

    if let Err(e) = create_dir_all(&data_dir) {
        eprintln!("Error creating local data directory: {}", e);
    }

    data_dir.to_path_buf()
}

#[cfg(all(test, not(feature = "deploy_desktop")))]
fn data_dir() -> PathBuf {
    "test_output".into()
}

#[cfg(all(not(test), not(feature = "deploy_desktop")))]
fn data_dir() -> PathBuf {
    "".into()
}
