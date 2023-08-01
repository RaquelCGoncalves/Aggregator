use rumqttc::{self, AsyncClient, QoS};
use std::error::Error;

pub async fn mqtt_subscriber(topic: &str, client: AsyncClient) -> Result<(), Box<dyn Error>> {
    if let Err(err) = client.subscribe(topic, QoS::ExactlyOnce).await {
        println!("Failed to subscribe to the topic: {}", err);
        return Err(Box::new(err));
    }

    Ok(())
}
