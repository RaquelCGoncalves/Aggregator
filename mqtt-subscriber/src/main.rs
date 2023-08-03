mod handler;
mod model;
mod mqtt_subscriber;
mod response;

use axum::{extract::State, routing::get, Extension, Json, Router};
use handler::facts_list_handler;
use model::DB;
use response::FactListResponse;
use rumqttc::{AsyncClient, MqttOptions};
use serde::Serialize;
use std::{
    error::Error,
    sync::{Arc, RwLock},
};

#[derive(Debug, Serialize)]
struct Fact {
    data: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<(dyn Error)>> {
    let v = Arc::new(RwLock::new(Vec::new()));
    let db: DB = v.clone();
    let db_clone = db.clone();

    // Set up the MQTT subscriber
    let broker_address = "127.0.0.1";
    let topic = "catsfacts/fact";
    let mqttoptions = MqttOptions::new("mqttsubscriber", broker_address, 1883);
    let (client, mut eventloop) = AsyncClient::new(mqttoptions.clone(), 10);

    mqtt_subscriber::mqtt_subscriber(topic, client.clone()).await?;

    // Spawn a task for the MQTT subscriber loop
    tokio::spawn(async move {
        loop {
            match eventloop.poll().await {
                Ok(e) => match e {
                    rumqttc::Event::Incoming(p) => {
                        if let rumqttc::Packet::Publish(m) = p {
                            let payload_str = String::from_utf8_lossy(&m.payload).to_string();
                            let mut v: std::sync::RwLockWriteGuard<'_, Vec<String>> =
                                db_clone.write().unwrap();
                            v.push(payload_str);
                        }
                    }
                    rumqttc::Event::Outgoing(_) => {}
                },
                Err(e) => {
                    println!("Error while polling mqtt eventloop: {}", e);
                    break;
                }
            }
        }
    });

    // Start the HTTP server
    let addr = "127.0.0.1:8080";
    let app = Router::new()
        .route("/facts", get(get_facts_list))
        .layer(Extension(db.clone()))
        .with_state(db);

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn get_facts_list(db: State<DB>) -> Json<FactListResponse> {
    // Call facts_list_handler function and return the JSON response
    facts_list_handler(db.0).await.unwrap()
}
