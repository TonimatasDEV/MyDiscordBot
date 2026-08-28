use std::{
    collections::HashMap,
    fs::File,
    io::BufReader,
    path::Path,
    sync::{Mutex, OnceLock},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildConfig {
    pub welcome_message_system: bool,
    pub welcome_message_id: u64,
    pub auto_role_system: bool,
    pub auto_role_id: u64,
    pub ticket_system: bool,
    pub ticket_channel: u64,
}

static CONFIGS: OnceLock<Mutex<HashMap<u64, GuildConfig>>> = OnceLock::new();

fn configs() -> &'static Mutex<HashMap<u64, GuildConfig>> {
    CONFIGS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn default_config() -> GuildConfig {
    GuildConfig {
        welcome_message_system: false,
        welcome_message_id: 0,
        auto_role_system: false,
        auto_role_id: 0,
        ticket_system: false,
        ticket_channel: 0,
    }
}

pub fn init_config_file(guild_id: u64) {
    let config_path = format!("config/{}.json", guild_id);
    if Path::new(&config_path).exists() {
        let _ = read_config_file(guild_id);
    } else {
        let _ = set_config(guild_id, default_config());
    }
}

fn read_config_file(guild_id: u64) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config_path = format!("config/{}.json", guild_id);
    let file = File::open(&config_path)?;
    let reader = BufReader::new(file);

    let server: GuildConfig = serde_json::from_reader(reader)?;

    let mut configs = configs().lock().unwrap();
    configs.insert(guild_id, server);

    Ok(())
}

fn save_config_file(
    guild_id: u64,
    guild_config: GuildConfig,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config_path = format!("config/{}.json", guild_id);

    let dir = config_path.rsplit_once('/').map(|(d, _)| d);
    if let Some(dir) = dir {
        std::fs::create_dir_all(dir)?;
    }

    let json = serde_json::to_string_pretty(&guild_config)?;
    std::fs::write(&config_path, json)?;

    Ok(())
}

pub fn get_config(guild_id: u64) -> GuildConfig {
    let configs = configs().lock().unwrap();
    configs
        .get(&guild_id)
        .cloned()
        .unwrap_or_else(|| default_config())
}

pub fn set_config(
    guild_id: u64,
    config: GuildConfig,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut configs = configs().lock().unwrap();
    save_config_file(guild_id, config.clone())?;
    configs.insert(guild_id, config);
    Ok(())
}
