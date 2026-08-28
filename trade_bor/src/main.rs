use config;
use data_recipient::data_recipient_algopack;
use std;

#[tokio::main] // Этот макрос запускает асинхронный движок Tokio
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Запуск торгового робота ===");
    let version = env!("CARGO_PKG_VERSION");
    println!("Текущая версия программы: {}", version);
    let config: &config::Config = config::get_config();

    let body = match data_recipient::data_recipient_algopack().await {
        Ok(body) => body,
        Err(e) => {
            eprintln!("Ошибка при получении данных: {}", e);
            return Err(e);
        }
    };
    println!("{:?}", body);

    Ok(())
}
