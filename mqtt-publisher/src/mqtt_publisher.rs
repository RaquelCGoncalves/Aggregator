use rumqttc::{self, AsyncClient, QoS};
use std::error::Error;

pub async fn mqtt_publisher(
    data: String,
    topic: &str,
    client: AsyncClient,
) -> Result<(), Box<dyn Error>> {
    println!("Publishing Fact");

    // Convert the data to bytes to pass as payload for MQTT
    let payload = data.as_bytes();

    // Publish the MQTT
    if let Err(err) = client
        .publish(topic, QoS::AtLeastOnce, false, payload)
        .await
    {
        println!("Failed to publish the MQTT message: {}", err);
        return Err(Box::new(err));
    }

    Ok(())
}
