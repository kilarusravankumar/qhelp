use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct Config {
    pub base_url: String,
    pub model: String,
}

fn get_config_path() -> PathBuf {
    dirs::config_dir()
        .expect("could not find the .config directory")
        .join("qhelp")
        .join("config.toml")
}
pub fn load_config() -> Result<Config, String> {
    let path = get_config_path();

    if !path.exists() {
        return Err(format!(
            "Config file is not found. \n Create one at:\n{}\nexample: config `base_url = http://localhost:<port>` and `model = <model_name>`",
            path.display()
        ));
    }

    let contents = std::fs::read_to_string(&path)
        .map_err(|e| format!("Could not read {}\nerr:{}", path.display(), e))?;

    let config: Config = toml::from_str(&contents)
        .map_err(|e| format!("invalid config at{}\nerr:{}", path.display(), e))?;

    Ok(config)
}
