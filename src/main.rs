use rand::{self, Rng};
use reqwest::{self, Client, Error};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    loop {
        let id: u8 = rand::thread_rng().gen_range(1..=4);
        update_value(id).await?;
    }
}

async fn update_value(id: u8) -> Result<(), Error> {
    let res: Value = reqwest::get(format!(
        "http://localhost:1026/v1/contextEntities/sensor{id}/attributes/capacidade"
    ))
    .await?
    .json()
    .await?;

    let capacity = res["attributes"][0]["value"]
        .as_str()
        .unwrap()
        .parse::<f64>()
        .unwrap() as u32;

    let data = generate_data(id, capacity);

    let client = Client::new();
    let res = client
        .post("http://localhost:1026/v1/updateContext")
        .json(&data)
        .send()
        .await?;

    let result: Value = res.json().await?;

    Ok(())
}

fn generate_data(id: u8, capacity: u32) -> Value {
    let mut rng = rand::thread_rng();
    let volume = rng.gen_range(0.0..=capacity as f64);
    let vazao = rng.gen_range(0.0..=capacity as f64);
    println!("volume e vazao do sensor{id} serão atualizadas para {volume} e {vazao}");
    json!(
        {
        "contextElements": [
        {
              "type": "Sensor",
              "isPattern": "false",
              "id": format!("sensor{id}"),
              "attributes": [
                  {
                      "name": "volume",
                      "type": "float",
                      "value": volume,
                  },
                  {
                      "name": "vazao",
                      "type": "float",
                      "value": vazao,
                  },
              ]
          }
    ],
    "updateAction": "UPDATE",
          }
      )
}
