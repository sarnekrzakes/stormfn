use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::{self, Write};

#[derive(Serialize, Deserialize)]
struct UnlockRequest {
    item_ids: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct UnlockResponse {
    success: bool,
    message: String,
}

struct StormFN {
    client: Client,
}

impl StormFN {
    fn new() -> Self {
        let client = Client::new();
        StormFN { client }
    }

    async fn unlock_items(&self, item_ids: Vec<String>) -> Result<UnlockResponse, Box<dyn Error>> {
        let request = UnlockRequest { item_ids };
        let response = self.client.post("https://api.fortnite.com/unlock")
            .json(&request)
            .send()
            .await?
            .json::<UnlockResponse>()
            .await?;
        Ok(response)
    }

    fn run(&self) {
        let item_ids = vec![
            "outfit1".to_string(),
            "emote1".to_string(),
            "pickaxe1".to_string(),
        ];

        let response = tokio::runtime::Runtime::new().unwrap().block_on(self.unlock_items(item_ids));

        match response {
            Ok(res) => {
                if res.success {
                    println!("Items unlocked successfully: {}", res.message);
                } else {
                    println!("Failed to unlock items: {}", res.message);
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn main() {
    let storm_fn = StormFN::new();
    storm_fn.run();
}