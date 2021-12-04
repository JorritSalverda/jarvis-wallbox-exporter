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
    pub id: i64,
    pub chargers: Vec<Charger>,   
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Charger {
    pub id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ChargerResponse {
    pub name: String,
    pub added_energy: f64,
    pub added_green_energy: f64,
    pub charging_power: f64,
    pub charging_time: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
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

    #[test]
    fn deserialize_token_response() {
        let token_response = fs::read_to_string("token_response.json".to_string()).expect("Can't read file");

        let token: Token =
            serde_json::from_str(&token_response).expect("JSON was not well-formatted");

        assert_eq!(token.jwt, "ABCD".to_string());
    }

    #[test]
    fn deserialize_charger_groups_response() {
        let token_response = fs::read_to_string("charger_groups_response.json".to_string()).expect("Can't read file");

        let charger_groups: ChargerGroupsResponse =
            serde_json::from_str(&token_response).expect("JSON was not well-formatted");

        assert_eq!(charger_groups.result.groups[0].id, 15);
        assert_eq!(charger_groups.result.groups[0].chargers[0].id, 304);
    }

    #[test]
    fn deserialize_charger_response() {
        let token_response = fs::read_to_string("charger_response.json".to_string()).expect("Can't read file");

        let charger: ChargerResponse =
            serde_json::from_str(&token_response).expect("JSON was not well-formatted");

        assert_eq!(charger.added_energy, 0.004);
    }
}
