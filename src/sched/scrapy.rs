use std::time::Duration;
use reqwest::Client;
use serde_json::Value;
use tokio::time;

pub async fn check_tpl_task() {
    let mut interval = time::interval(Duration::from_millis(15000));
    loop {
        interval.tick().await;
        let check_result = check_tpl();
        check_result.await;
    }
}

pub async fn check_tpl() {
    let url = "https://alternativeto.net/_next/data/ld18YV7eEajAG5N7Htuus/browse/platform/windows.json?p=2&browse=platform&appList=windows";
    let client = Client::new();
    let response = client
        .get(url)
        .body("{}")
        .send()
        .await;
    match response {
        Ok(r) => {
            let text_response = r.text().await;
            match text_response {
                Ok(text) => {
                    let _json_value: Value = serde_json::from_str(&text).expect("Failed to parse JSON");
                    
                },
                Err(_) => {},
            }
        },
        Err(e) => {
            println!("Error: {}", e) 
        },
    }


}



