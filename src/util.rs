pub const CLIENT_ID_ENV_VAR: &str = "TWITCH_VIEWERS_CLIENT_ID";

pub fn get_api_key(key: &str) -> String {
    std::env::var(key)
        .ok()
        .or_else(|| {
            use std::collections::HashMap;
            let file = std::fs::read_to_string(".env").expect(".env file to exist");
            let map = file
                .lines()
                .map(|s| s.split('='))
                .filter_map(|mut s| s.next().and_then(|k| s.next().map(|v| (k, v))))
                .collect::<HashMap<_, _>>();
            map.get(key).map(ToString::to_string)
        })
        .unwrap_or_else(|| panic!("{} must be set", key))
}

pub struct Resources {
    pub icon: std::path::PathBuf,
    pub css: std::path::PathBuf,
}

impl Resources {
    pub fn load() -> Self {
        let base = directories::ProjectDirs::from("com.github", "museun", "viewers")
            .unwrap_or_else(|| {
                log::error!("cannot load resources, invalid $HOME directory");
                std::process::exit(1);
            });

        let icon = base.data_dir().join("glitch.png");
        let css = base.data_dir().join("style.css");

        match (std::fs::metadata(&icon), std::fs::metadata(&css)) {
            (Ok(icon_), Ok(css_)) => {
                if !icon_.is_file() {
                    log::error!("{} isn't a valid file", icon.display())
                }
                if !css_.is_file() {
                    log::error!("{} isn't a valid file", css.display())
                }
            }
            (Err(..), Err(..)) => {
                log::error!("glitch.png is missing at: {}", icon.display());
                log::error!("style.css is missing at: {}", css.display());
                std::process::exit(1);
            }
            (.., Err(..)) => {
                log::error!("style.css is missing at: {}", css.display());
                std::process::exit(1);
            }
            (Err(..), ..) => {
                log::error!("glitch.png is missing at: {}", icon.display());
                std::process::exit(1);
            }
        }

        Self { icon, css }
    }
}
