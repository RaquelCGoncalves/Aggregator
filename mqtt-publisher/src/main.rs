use rumqttc::{AsyncClient, MqttOptions};
use std::{error::Error, time::Duration};
use tokio::time::interval;
mod mqtt_publisher;
mod request;

#[derive(Debug)]
// Struct to hold the fetched data
struct Fact {
    data: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<(dyn Error)>> {
    let num_requests = 5;
    let interval_seconds = 5;
    let mut interval = interval(Duration::from_secs(interval_seconds));
    let mut request_counter = 0;

    // Set up MQTT broker connection options
    let broker_address = "127.0.0.1";
    let topic = "catsfacts/fact";
    let mqttoptions = MqttOptions::new("mqttpublisher", broker_address, 1883);

    // Create an MQTT client
    let (client, mut eventloop) = AsyncClient::new(mqttoptions.clone(), 10);

    tokio::spawn(async move {
        loop {
            match eventloop.poll().await {
                Ok(_) => {}
                Err(e) => {
                    println!("Error while polling mqtt eventloop: {}", e);
                    break;
                }
            }
        }
    });

    loop {
        interval.tick().await;

        if request_counter == num_requests {
            break;
        }

        // Request to fetch data from the website
        let request_result = request::request().await?;

        // Create a Fact struct to hold the fetched data
        let fact = Fact {
            data: request_result,
        };

        mqtt_publisher::mqtt_publisher(fact.data, topic, client.clone()).await?;

        request_counter += 1;
    }
    Ok(())
}
