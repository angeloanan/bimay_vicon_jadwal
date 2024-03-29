use indoc::{formatdoc, indoc};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub token: String,
    pub zoom_username: String,
}

pub async fn read_config_or_exit() -> Result<Config, String> {
    let base_dir = directories::BaseDirs::new().unwrap();
    let config_dir = base_dir.config_dir();
    let config_path = config_dir.join("BimayViconJadwal").join("config.toml");

    if config_path.exists() {
        let credentials_string = tokio::fs::read_to_string(&config_path).await.unwrap();
        let config = toml::from_str::<Config>(credentials_string.as_str()).expect(indoc!{"
            > Unable to parse `config.toml`.
            If you've just upgraded to a new version, please delete the old `config.toml` and re-run the program.
        "});

        if config.token == "CHANGEME" {
            Err(format!(
                "Please update your credentials in {}",
                config_path.to_str().unwrap()
            ))
        } else {
            Ok(config)
        }
    } else {
        write_config_template(&config_path).await;
        Err(formatdoc!(
            "
                > Please fill in the config file with your credentials.
                The config file is located at {}.
            ",
            config_path.to_str().unwrap()
        ))
    }
}

pub async fn write_config_template(path: &std::path::Path) {
    let config_template = include_bytes!("./template/config.toml");

    tokio::fs::create_dir_all(path.parent().unwrap())
        .await
        .expect("Failed to create config directory");
    tokio::fs::write(path, config_template)
        .await
        .expect("Failed to write config template");
}
