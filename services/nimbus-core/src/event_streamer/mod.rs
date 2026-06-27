use crate::proto_gen::service_discovery::v1::{
    PostStatsRequest,
    nimbus_core_service_discovery_service_client::NimbusCoreServiceDiscoveryServiceClient,
};
use std::time::Duration;
use tokio::{
    sync::mpsc,
    time::{interval, sleep},
};
use tokio_stream::wrappers::ReceiverStream;

pub async fn start_streaming() {
    let uri = "http://127.0.0.1:50051";

    // Outer loop manages connection/re-connection state
    loop {
        println!("Attempting to connect to the Service Registry...");

        let mut client = match NimbusCoreServiceDiscoveryServiceClient::connect(uri).await {
            Ok(cl) => {
                println!("Successfully connected and registered with server.");
                cl
            }
            Err(e) => {
                eprintln!(
                    "Failed to connect to the Service Registry: {}. Retrying in 4 seconds...",
                    e
                );
                sleep(Duration::from_secs(4)).await;
                continue; // Jump back to the top of outer loop to retry connection
            }
        };

        // Inner loop manages the 15-second interval once connected

        let mut ticker = interval(Duration::from_secs(15));
        // 1. Create the channel
        //
        let (tx, rx) = mpsc::channel(10);

        tokio::spawn(async move {
            loop {
                if tx
                    .send(PostStatsRequest {
                        address: "".to_string(),
                    })
                    .await
                    .is_err()
                {
                    break;
                }

                ticker.tick().await;
            }
        });

        let outbound_stream = ReceiverStream::new(rx);

        // Execute the client streaming RPC
        // (Replace `monitor_heartbeat` with your actual method name)
        match client.post_stats(outbound_stream).await {
            Ok(response) => {
                println!("Stream processed successfully: {:?}", response.into_inner());
            }
            Err(status) => {
                eprintln!(
                    "Connection error encountered during stream (gRPC code: {:?}, msg: {}).",
                    status.code(),
                    status.message()
                );

                // Break out of the 15-second loop because connection is dead.
                // This drops down to the outer loop, triggering a re-connection attempt.
                break;
            }
        }

        // If we broke out of the inner loop, it means the connection closed.
        // We wait a brief moment (or fallback directly) to avoid hammering the socket.
        println!("Connection lost. Dropping back to connection manager...");
        sleep(Duration::from_secs(1)).await;
    }
}
