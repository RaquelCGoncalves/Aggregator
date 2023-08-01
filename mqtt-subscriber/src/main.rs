use rumqttc::{AsyncClient, MqttOptions};
use std::error::Error;

mod mqtt_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<(dyn Error)>> {
    let broker_address = "127.0.0.1";
    let topic = "catsfacts/fact";
    let mqttoptions = MqttOptions::new("mqttsubscriber", broker_address, 1883);
    let (client, mut eventloop) = AsyncClient::new(mqttoptions.clone(), 10);
    let mut v: Vec<String> = Vec::new();
    mqtt_subscriber::mqtt_subscriber(topic, client.clone()).await?;

    loop {
        match eventloop.poll().await {
            Ok(e) => match e {
                rumqttc::Event::Incoming(p) => {
                    if let rumqttc::Packet::Publish(m) = p {
                        let payload_str = String::from_utf8_lossy(&m.payload).to_string();
                        v.push(payload_str.clone());
                        println!("{:?}", payload_str);
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
    Ok(())
}
