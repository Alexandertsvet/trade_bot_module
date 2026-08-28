use serde::Deserialize;
use std::env;
use std::path::Path;
use std::sync::OnceLock;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub api_key_algopack: String,
}

fn load_env_file() {
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let env_path = Path::new(&manifest_dir).join("../.env");
        if dotenvy::from_path(&env_path).is_ok() {
            return;
        }
    }
    if let Err(e) = dotenvy::dotenv() {
        if !e.not_found() {
            eprintln!("Ошибка при разборе файла .env: {}", e);
        } else {
            println!("Предупреждение: Файл .env не найден. Используются системные переменные.");
        }
    }
}

pub fn get_config() -> &'static Config {
    static CONFIG: OnceLock<Config> = OnceLock::new();
    CONFIG.get_or_init(|| {
        load_env_file();
        envy::from_env::<Config>().unwrap_or_else(|err| {
            panic!("Критическая ошибка валидации переменных окружения: {}", err);
        })
    })
}
