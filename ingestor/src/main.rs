use common::IoTMessage;
use dotenv::dotenv;
use log::{error, info};
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::time::Duration;
use tokio::time;

#[derive(Debug)]
struct IngestorConfig {
    mqtt_host: String,
    mqtt_port: u16,
    database_url: String,
}

impl IngestorConfig {
    fn from_env() -> Self {
        Self {
            mqtt_host: env::var("MQTT_HOST").unwrap_or_else(|_| "localhost".to_string()),
            mqtt_port: env::var("MQTT_PORT")
                .unwrap_or_else(|_| "1883".to_string())
                .parse()
                .unwrap(),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    let config = IngestorConfig::from_env();
    info!("Starting NordicPulse Ingestor...");
    info!("Connecting to Database...");

    // DB Connection
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    info!("Connected to TimescaleDB.");

    // MQTT connection
    let mut mqttoptions = MqttOptions::new("ingestor-main", &config.mqtt_host, config.mqtt_port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    // Subscribe to all telemetry
    client
        .subscribe("nordic_pulse/telemetry/#", QoS::AtLeastOnce)
        .await?;

    info!("Subscribed to nordic_pulse/telemetry/#");

    // Batch configs
    let batch_size = 1000;
    let max_delay = Duration::from_secs(1);

    let mut batch = Vec::with_capacity(batch_size);
    let mut last_flush = time::Instant::now();

    loop {
        // We poll with a timeout so we can flush on time even if no messages
        // But rumqttc poll is not timeout-based easily unless we wrap it.
        // Simplified: just wait for event. If traffic is high, we flush by size.
        // If traffic is low, we might delay.
        // Better: select with timeout.

        tokio::select! {
             notification = eventloop.poll() => {
                 match notification {
                     Ok(Event::Incoming(Packet::Publish(p))) => {
                         if let Ok(msg) = serde_json::from_slice::<IoTMessage>(&p.payload) {
                             batch.push(msg);
                         }
                     }
                     Ok(_) => {}, // Other events
                     Err(e) => {
                         error!("MQTT Connection error: {:?}", e);
                         time::sleep(Duration::from_secs(1)).await;
                     }
                 }
             }
             _ = time::sleep(Duration::from_millis(100)) => {
                 // Check flush on timeout tick
             }
        }

        if batch.len() >= batch_size || last_flush.elapsed() >= max_delay {
            if !batch.is_empty() {
                if let Err(e) = insert_batch(&pool, &batch).await {
                    error!("Failed to insert batch: {:?}", e);
                } else {
                    info!("Inserted {} metrics", batch.len());
                }
                batch.clear();
            }
            last_flush = time::Instant::now();
        }
    }
}

async fn insert_batch(pool: &sqlx::PgPool, batch: &[IoTMessage]) -> Result<(), sqlx::Error> {
    // UNNEST optimization for bulk insert (High Performance)
    // We unnest arrays of values.

    let mut times = Vec::with_capacity(batch.len());
    let mut device_ids = Vec::with_capacity(batch.len());
    let mut device_types = Vec::with_capacity(batch.len());
    let mut locations = Vec::with_capacity(batch.len());
    let mut values = Vec::with_capacity(batch.len());
    let mut units = Vec::with_capacity(batch.len());

    for msg in batch {
        times.push(msg.timestamp);
        device_ids.push(msg.device_id.clone());
        device_types.push(msg.device_type.to_string());
        locations.push(msg.location.clone());
        values.push(msg.value);
        units.push(msg.unit.clone());
    }

    sqlx::query!(
        r#"
        INSERT INTO iot_data (time, device_id, device_type, location, value, unit)
        SELECT * FROM UNNEST($1::timestamptz[], $2::text[], $3::text[], $4::text[], $5::float8[], $6::text[])
        "#,
        &times,
        &device_ids,
        &device_types,
        &locations,
        &values,
        &units
    )
    .execute(pool)
    .await?;

    Ok(())
}
