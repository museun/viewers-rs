use std::path::Path;

fn file_exists(s: impl AsRef<Path>) -> bool {
    std::fs::metadata(s)
        // FIXME: what if its a dir?
        .map(|s| s.is_file())
        .unwrap_or_default()
}

fn copy_data(base: impl AsRef<Path>) {
    let base = base.as_ref();
    std::fs::create_dir_all(base).expect("create data directory");

    for from in FILES {
        let f = std::path::PathBuf::from(from)
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();

        let to = base.join(&f);
        if !file_exists(&f) {
            eprintln!("copying {} to {}", from, to.display());
            let _ = std::fs::copy(from, to).expect("copy resource");
        }
    }
}

const FILES: &[&str] = &[
    "./resources/glitch.png", //
    "./resources/style.css",  //
];

fn main() {
    for file in FILES {
        assert!(file_exists(file));
    }
    let base = directories::ProjectDirs::from("com.github", "museun", "viewers")
        .expect("valid $HOME directory");
    copy_data(base.data_dir())
}
