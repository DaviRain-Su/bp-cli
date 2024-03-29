use crate::config::Config;
use crate::error::Error;

pub fn get_config() -> Result<Config, Error> {
    let home_path = dirs::home_dir().ok_or(Error::Custom("can't open home dir".to_string()))?;
    let pomm_config_path = home_path.join(".config").join("bp-cli");
    let config_path = pomm_config_path.join("config.toml");

    let config_str = std::fs::read_to_string(config_path)
        .map_err(|e| Error::Custom(format!("read file content failed: Error({})", e)))?;
    let config: Config = toml::from_str(&config_str)
        .map_err(|e| Error::Custom(format!("parse toml failed: Error({})", e)))?;
    // log::info!("config: {:?}", config);

    Ok(config)
}
