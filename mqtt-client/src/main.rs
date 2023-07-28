use std::{error::Error, time::Duration};
use rumqttc::{MqttOptions, AsyncClient, Incoming, ConnectReturnCode, QoS};

mod request;
mod mqttpublisher;

#[derive(Debug)]
// Struct to hold the fetched data
struct Fact {
    data: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
     // Request to fetch data from the website
    let request_result = request::request().await?;
    println!("{:?}", &request_result);

    // Create a Fact struct to hold the fetched data
    let fact = Fact {
        data: request_result, 
    };

     // Set up MQTT broker connection options
    let broker_address = "127.0.0.1";
    let topic = "catsfacts/fact";
    let mqttoptions = MqttOptions::new("mqttpublisher", broker_address, 1883);
    
    // Create an MQTT client
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    // Spawn an asynchronous MQTT publishing task
    tokio::spawn(mqttpublisher::mqtt_publisher(fact.data, topic, client.clone()));
    // Start the event loop to handle MQTT events
    loop {
        match eventloop.poll().await {
            Ok(rumqttc::Event::Incoming(Incoming::ConnAck(ack))) => {
                if ack.code == ConnectReturnCode::Success {
                    println!("Successfully connected to mqtt broker");
                    if (client.subscribe(topic, QoS::ExactlyOnce).await).is_err() {
                        println!("Could not subscribe");
                    }
                } else {
                    println!("Could not connect to mqtt broker: {:?}", ack.code);
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }
            Ok(rumqttc::Event::Incoming(Incoming::Disconnect)) => {
                println!("Disconnected from mqtt broker");
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
            Ok(_) => {}
            Err(e) => {
                println!("Error while polling mqtt eventloop: {}", e);
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }
    }
         
}