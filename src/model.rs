use jarvis_lib::config_client::SetDefaults;
use jarvis_lib::model::EntityType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub location: String,
    pub entity_type: EntityType,
    pub entity_name: String,
}

impl SetDefaults for Config {
    fn set_defaults(&mut self) {}
}


// https://api.wall-box.com/auth/token/user

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub jwt: String,    
}

// https://api.wall-box.com/v4/chargers/groups
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChargerGroupsResponse {
    pub result: ChargerGroupsResult,    
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChargerGroupsResult {
    pub groups: Vec<ChargerGroup>,    
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChargerGroup {
    pub id: int,
    pub chargers: Vec<Charger>,   
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snakeCase")]
pub struct Charger {
    pub id: int,
    pub name: String,
    pub addedEnergy: f64,
    pub addedGreenEnergy: f64,
    pub chargingTime: f64,
}





#[cfg(test)]
mod tests {
    use super::*;
    use jarvis_lib::config_client::{ConfigClient, ConfigClientConfig};
    use jarvis_lib::model::EntityType;

    #[test]
    fn read_config_from_file_returns_deserialized_test_file() {
        let config_client =
            ConfigClient::new(ConfigClientConfig::new("test-config.yaml".to_string()).unwrap());

        let config: Config = config_client.read_config_from_file().unwrap();

        assert_eq!(config.location, "My Home".to_string());
        assert_eq!(config.entity_type, EntityType::Device);
        assert_eq!(config.entity_name, "Wallbox PulsarPlus".to_string());
    }
}
