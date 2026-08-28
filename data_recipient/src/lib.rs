use config;
use config::Config;
use reqwest::header;
use std;

pub async fn data_recipient_algopack() -> Result<String, Box<dyn std::error::Error>> {
    let config: &config::Config = config::get_config();
    let client = reqwest::Client::builder().build()?;
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Accept", "application/json".parse()?);
    let auth_str = format!("Bearer {}", config.api_key_algopack);
    headers.insert("Authorization", auth_str.parse()?);

    let ticker = "GAZP".to_string();
    let url = format!(
        "https://apim.moex.com/iss/datashop/algopack/eq/tradestats/{}.json?latest=1",
        ticker
    );

    let request = client.request(reqwest::Method::GET, url).headers(headers);
    let response = match request.send().await {
        Ok(res) => res,
        Err(err) => {
            eprintln!(
                "[ОШИБКА СЕТИ] Не удалось отправить запрос для {}: {}",
                ticker, err
            );
            return Err(Box::new(err));
        }
    };
    let status = response.status();
    if !status.is_success() {
        let error_body = response
            .text()
            .await
            .unwrap_or_else(|_| "Не удалось прочитать тело ошибки".to_string());

        eprintln!(
            "[ОШИБКА API ALGOPACK] Сервер вернул код ответа: {}.\nДетали от сервера: {}",
            status, error_body
        );

        return Err(format!("ALGOPACK Error (HTTP {}): {}", status, error_body).into());
    }

    let body = match response.text().await {
        Ok(text) => text,
        Err(err) => {
            eprintln!("[ОШИБКА ЧТЕНИЯ] Не удалось прочитать тело ответа: {}", err);
            return Err(Box::new(err));
        }
    };
    Ok(body)
}
