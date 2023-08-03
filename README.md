# Aggregator

This Rust project demonstrates the integration of an MQTT publisher and an HTTP server with an MQTT subscriber,

1. MQTT Publisher:
-Publishes data fetched from a website to an MQTT broker.
-Uses rumqttc library for asynchronous MQTT communication.
-Publishes data to the "catsfacts/fact" topic every 5 seconds.

2.HTTP Server and MQTT Subscriber:

-Sets up an HTTP server using the Axum framework.
-Provides an API endpoint ("/facts") to retrieve a list of facts.
-Maintains an internal state to store received MQTT messages.
-The MQTT subscriber listens for messages on the "catsfacts/fact" topic and stores them in the shared state.
-The server returns the list of facts as a JSON response when the "/facts" endpoint is accessed.

Overall, this project demonstrates an integration between an MQTT publisher, HTTP server, and MQTT subscriber, enabling data retrieval from a website and its availability through an HTTP API.