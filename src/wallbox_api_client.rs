use crate::model::{Config,Token,ChargerGroupsResponse, ChargerResponse};
use jarvis_lib::measurement_client::MeasurementClient;
use jarvis_lib::model::{Measurement, MetricType, Sample, SampleType};

use chrono::Utc;
use std::env;
use std::error::Error;
use uuid::Uuid;
use http_auth_basic::Credentials;

pub struct WallboxApiClientConfig {
    username: String,
    password: String,
}

impl WallboxApiClientConfig {
    pub fn new(username: String, password: String) -> Result<Self, Box<dyn Error>> {
        println!(
            "WallboxApiClientConfig::new(username: {}, password: ***)",
            username
        );

        Ok(Self { username, password })
    }

    pub fn from_env() -> Result<Self, Box<dyn Error>> {
        let username = env::var("USERNAME")?;

        let password = env::var("PASSWORD")?;

        Self::new(username, password)
    }
}

pub struct WallboxApiClient {
    config: WallboxApiClientConfig,
}

impl MeasurementClient<Config> for WallboxApiClient {
    fn get_measurement(
        &self,
        config: Config,
        _last_measurement: Option<Measurement>,
    ) -> Result<Measurement, Box<dyn Error>> {
        println!("Reading measurement from Wallbox devices...");

        let mut measurement = Measurement {
            id: Uuid::new_v4().to_string(),
            source: String::from("jarvis-wallbox-exporter"),
            location: config.location.clone(),
            samples: Vec::new(),
            measured_at_time: Utc::now(),
        };

        println!("Retrieving token for api...");
        let token: Token = self.get_token()?;

        println!("Discovering chargers...");
        let charger_groups = self.get_charger_groups(&token)?;

        for group in charger_groups.result.groups.iter() {
            for charger in group.chargers.iter() {
                let charger_response  = self.get_charger(&token, charger.id)?;

                // counter
                measurement.samples.push(Sample {
                    entity_type: config.entity_type,
                    entity_name: config.entity_name.clone(),
                    sample_type: SampleType::ElectricityConsumption,
                    sample_name: charger_response.name.clone(),
                    metric_type: MetricType::Counter,
                    value: charger_response.added_energy * 3600.0 * 1000.0,
                });

              //   // gauge
              //   measurement.samples.push(Sample {
              //     entity_type: config.entity_type,
              //     entity_name: config.entity_name.clone(),
              //     sample_type: SampleType::ElectricityConsumption,
              //     sample_name: charger_response.name.clone(),
              //     metric_type: MetricType::Gauge,
              //     value: charger_response.charging_power * 1000.0,
              // });
            }
        }

        // match last_measurement {
        //     Some(_) => {
        //         // measurement.samples = self.sanitize_samples(measurement.samples, lm.samples)
        //     }
        //     None => {}
        // }

        println!("Read measurement from Wallbox chargers");

        Ok(measurement)
    }
}

impl WallboxApiClient {
    pub fn new(config: WallboxApiClientConfig) -> Self {
        Self { config }
    }

    // https://api.wall-box.com/auth/token/user
    fn get_token(&self) -> Result<Token, Box<dyn Error>> {
        let credentials = Credentials::new(&self.config.username, &self.config.password);
        let credentials = credentials.as_http_header();
        
        let client = reqwest::blocking::Client::new();
        let token = client.post("https://api.wall-box.com/auth/token/user")
            .header("Content-type", "application/x-www-form-urlencoded")
            .header("Authorization", credentials)
            .send()?
            .json::<Token>()?;

        Ok(token)
    }

    // https://api.wall-box.com/v3/chargers/groups
    fn get_charger_groups(&self, token: &Token) -> Result<ChargerGroupsResponse, Box<dyn Error>> {
        let client = reqwest::blocking::Client::new();
        let charger_groups_response = client.get("https://api.wall-box.com/v3/chargers/groups")
            .header("Authorization", format!("{} {}", "Bearer", token.jwt))
            .send()?
            .json::<ChargerGroupsResponse>()?;

        Ok(charger_groups_response)
    }
    
    // https://api.wall-box.com/v2/charger/<charger id>
    fn get_charger(&self, token: &Token, charger_id: i64) -> Result<ChargerResponse, Box<dyn Error>> {
        let client = reqwest::blocking::Client::new();
        let charger_response = client.get(format!("https://api.wall-box.com/chargers/status/{}", charger_id))
            .header("Authorization", format!("{} {}", "Bearer", token.jwt))
            .send()?
            .json::<ChargerResponse>()?;

        Ok(charger_response)
    }

    // fn sanitize_samples(
    //     &self,
    //     current_samples: Vec<Sample>,
    //     last_samples: Vec<Sample>,
    // ) -> Vec<Sample> {
    //     let mut sanitized_samples: Vec<Sample> = Vec::new();

    //     for current_sample in current_samples.into_iter() {
    //         // check if there's a corresponding sample in lastSamples and see if the difference with it's value isn't too large
    //         let mut sanitize = false;
    //         for last_sample in last_samples.iter() {
    //             if current_sample.entity_type == last_sample.entity_type
    //                 && current_sample.entity_name == last_sample.entity_name
    //                 && current_sample.sample_type == last_sample.sample_type
    //                 && current_sample.sample_name == last_sample.sample_name
    //                 && current_sample.metric_type == last_sample.metric_type
    //             {
    //                 if current_sample.metric_type == MetricType::Counter
    //                     && current_sample.value / last_sample.value > 1.1
    //                 {
    //                     sanitize = true;
    //                     println!("Value for {} is more than 10 percent larger than the last sampled value {}, keeping previous value instead", current_sample.sample_name, last_sample.value);
    //                     sanitized_samples.push(last_sample.clone());
    //                 }

    //                 break;
    //             }
    //         }

    //         if !sanitize {
    //             sanitized_samples.push(current_sample);
    //         }
    //     }

    //     sanitized_samples
    // }
}
