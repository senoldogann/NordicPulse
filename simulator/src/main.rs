use chrono::{Timelike, Utc};
use common::{DeviceType, IoTMessage};
use dotenv::dotenv;
use log::{error, info};
use rand::Rng;
use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::env;
use std::f64::consts::PI;
use std::time::Duration;
use tokio::task;

#[derive(Debug, Clone)]
enum DeviceState {
    Solar {
        efficiency: f64,
    },
    EV {
        is_charging: bool,
        remaining_ticks: u32,
        base_power: f64,
    },
    Heater {
        current_temp: f64,
        target_temp: f64,
        max_power: f64,
    },
}

struct SimulatorConfig {
    mqtt_host: String,
    mqtt_port: u16,
    device_count: usize,
}

impl SimulatorConfig {
    fn from_env() -> Self {
        Self {
            mqtt_host: env::var("MQTT_HOST").unwrap_or_else(|_| "localhost".to_string()),
            mqtt_port: env::var("MQTT_PORT")
                .unwrap_or_else(|_| "1883".to_string())
                .parse()
                .unwrap(),
            device_count: env::var("DEVICE_COUNT")
                .unwrap_or_else(|_| "1000".to_string())
                .parse()
                .unwrap(),
        }
    }
}

struct PopulationCenter {
    lat: f64,
    lon: f64,
    weight: f64,
    spread: f64,
}

const CLUSTERS: [PopulationCenter; 4] = [
    PopulationCenter {
        lat: 60.1699,
        lon: 24.9384,
        weight: 0.45,
        spread: 0.3,
    }, // Helsinki
    PopulationCenter {
        lat: 61.4978,
        lon: 23.7608,
        weight: 0.30,
        spread: 0.6,
    }, // Tampere/Turku
    PopulationCenter {
        lat: 65.0121,
        lon: 25.4651,
        weight: 0.15,
        spread: 0.4,
    }, // Oulu
    PopulationCenter {
        lat: 66.5000,
        lon: 25.7000,
        weight: 0.10,
        spread: 2.0,
    }, // Rural North
];

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    info!("Starting NordicPulse Simulator (Organic Mode + Geospatial Clustered)...");
    let config = SimulatorConfig::from_env();

    // MQTT Setup
    let mut mqttoptions = MqttOptions::new("simulator-main", &config.mqtt_host, config.mqtt_port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    // Spawn Event Loop
    task::spawn(async move {
        while let Ok(_notification) = eventloop.poll().await {
            // Keep loop running
        }
    });

    // Spawn Devices
    let mut tasks = Vec::with_capacity(config.device_count);

    for i in 0..config.device_count {
        let client = client.clone();
        let id = format!("dev_{:04}", i);
        let mut rng = rand::thread_rng();

        // Randomly assign device type and initial state
        let (device_type, state) = match i % 4 {
            0 => {
                let eff = rng.gen_range(0.8..1.0);
                (
                    DeviceType::SolarPanel,
                    DeviceState::Solar { efficiency: eff },
                )
            }
            1 => (
                DeviceType::EVCharger,
                DeviceState::EV {
                    is_charging: false,
                    remaining_ticks: 0,
                    base_power: 11.0,
                },
            ),
            2 => (
                DeviceType::Sauna,
                DeviceState::Heater {
                    current_temp: 20.0,
                    target_temp: 20.0,
                    max_power: 7.0,
                },
            ),
            _ => (
                DeviceType::HeatPump,
                DeviceState::Heater {
                    current_temp: 20.0,
                    target_temp: 22.0,
                    max_power: 3.5,
                },
            ),
        };

        // Clustered Geospatial Distribution
        let location = generate_clustered_location(&mut rng);

        let task = task::spawn(async move {
            run_device_loop(client, id, device_type, state, location).await;
        });
        tasks.push(task);
    }

    info!("Spawned {} organic clustered device tasks.", tasks.len());

    for t in tasks {
        let _ = t.await;
    }
}

fn generate_clustered_location(rng: &mut impl Rng) -> String {
    // 1. Pick a cluster based on weight
    let mut p = rng.gen_range(0.0..1.0);
    let mut selected_cluster = &CLUSTERS[0];
    for cluster in CLUSTERS.iter() {
        if p < cluster.weight {
            selected_cluster = cluster;
            break;
        }
        p -= cluster.weight;
    }

    // 2. Generate Gaussian offset using Box-Muller transform
    let u1: f64 = rng.gen_range(0.0..1.0);
    let u2: f64 = rng.gen_range(0.0..1.0);
    let z0 = (-2.0 * u1.ln()).sqrt() * (2.0 * PI * u2).cos();
    let z1 = (-2.0 * u1.ln()).sqrt() * (2.0 * PI * u2).sin();

    let lat = selected_cluster.lat + z0 * selected_cluster.spread;
    let lon = selected_cluster.lon + z1 * selected_cluster.spread;

    // Clamp to Finland's general vicinity just in case
    let lat = lat.clamp(59.0, 71.0);
    let lon = lon.clamp(19.0, 32.0);

    format!("{:.4},{:.4}", lat, lon)
}

async fn run_device_loop(
    client: AsyncClient,
    id: String,
    dtype: DeviceType,
    mut state: DeviceState,
    location: String,
) {
    let start_delay = rand::thread_rng().gen_range(0..5000);
    tokio::time::sleep(Duration::from_millis(start_delay)).await;

    loop {
        // High Availability: 99.95% online
        let is_online = rand::thread_rng().gen_bool(0.9995);

        if is_online {
            let value = update_state_and_get_value(&mut state);
            let unit = match dtype {
                DeviceType::SolarPanel => "kWh".to_string(),
                _ => "kW".to_string(),
            };

            let msg = IoTMessage::new(id.clone(), dtype.clone(), value, unit, location.clone());
            let topic = format!("nordic_pulse/telemetry/{}/{}", dtype, id);
            let payload = serde_json::to_vec(&msg).unwrap();

            if let Err(e) = client
                .publish(&topic, QoS::AtLeastOnce, false, payload)
                .await
            {
                error!("Publish error for {}: {:?}", id, e);
            }
        }

        // Fixed tick rate (2-4 seconds) for smoother trends
        let sleep_duration = rand::thread_rng().gen_range(2000..4000);
        tokio::time::sleep(Duration::from_millis(sleep_duration)).await;
    }
}

fn update_state_and_get_value(state: &mut DeviceState) -> f64 {
    let mut rng = rand::thread_rng();
    let now = Utc::now();
    let hour = now.hour() as f64 + (now.minute() as f64 / 60.0);

    match state {
        DeviceState::Solar { efficiency } => {
            // Strict Day/Night cycle + Clouds (noise)
            if hour < 6.0 || hour > 18.0 {
                0.0
            } else {
                let t = (hour - 6.0) * (PI / 12.0);
                let base = t.sin();
                let cloud_noise = rng.gen_range(0.85..1.0); // Simple cloud factor
                (base * 5.0 * (*efficiency) * cloud_noise).max(0.0).min(5.0)
            }
        }
        DeviceState::EV {
            is_charging,
            remaining_ticks,
            base_power,
        } => {
            if *is_charging {
                if *remaining_ticks > 0 {
                    *remaining_ticks -= 1;
                    // Brownian drift around base power
                    let drift = rng.gen_range(-0.1..0.1);
                    (*base_power + drift).max(10.5).min(11.5)
                } else {
                    *is_charging = false;
                    0.0
                }
            } else {
                // 0.5% chance to start a charging session
                if rng.gen_bool(0.005) {
                    *is_charging = true;
                    *remaining_ticks = rng.gen_range(30..120); // 1-8 minutes of simulation data
                    *base_power + rng.gen_range(-0.1..0.1)
                } else {
                    0.0
                }
            }
        }
        DeviceState::Heater {
            current_temp,
            target_temp,
            max_power,
        } => {
            // Randomly update target temp to simulate thermostat
            if rng.gen_bool(0.01) {
                *target_temp = if rng.gen_bool(0.5) { 22.0 } else { 18.0 };
            }

            let temp_diff = *target_temp - *current_temp;

            // Thermal inertia: Heat up slowly if target > current
            if temp_diff > 0.1 {
                *current_temp += 0.05;
                // Power proportional to delta
                (*max_power * (temp_diff / 5.0).min(1.0)).max(0.5)
            } else if temp_diff < -0.1 {
                *current_temp -= 0.02; // Cooldown is slower
                0.0
            } else {
                // Maintenance load
                rng.gen_range(0.1..0.3)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc; // Assuming Utc is available from chrono
    use std::f64::consts::PI; // Assuming PI is available from std::f64::consts

    #[test]
    fn test_solar_logic_night() {
        let mut state = DeviceState::Solar { efficiency: 1.0 };
        // We can't easily mock Utc::now() without more crates, but we can test the math
        // if we refactor or just verify it doesn't crash and returns 0.0 at night if possible.
        // For now, let's just test that it's within bounds.
        let val = update_state_and_get_value(&mut state);
        assert!(val >= 0.0 && val <= 5.0);
    }

    #[test]
    fn test_heater_inertia() {
        let mut state = DeviceState::Heater {
            current_temp: 20.0,
            target_temp: 25.0,
            max_power: 7.0,
        };
        let val = update_state_and_get_value(&mut state);
        // Should be heating since target > current
        assert!(val > 0.0);
        if let DeviceState::Heater { current_temp, .. } = state {
            assert!(current_temp > 20.0);
        }
    }
}
