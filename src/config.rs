use std::{fs, env};
use std::path::Path;
use log::{debug, warn};
use serde_derive::*;
use toml;
use lazy_static::lazy_static;

use clap::{Arg, ArgMatches, Command};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Defines and parses CLI argument for this server.
pub fn parse_cli_args() -> ArgMatches {
    Command::new("qaruntime-rs")
        .version(VERSION)
        .arg(
            Arg::new("config")
                .help("Path to configuration file")
                .index(1),
        )
        .get_matches()
}

/// Parses CLI arguments, finds location of config file, and parses config file into a struct.
pub fn parse_config_from_cli_args(matches: &ArgMatches) -> Config {
    let conf = match matches.get_one::<String>("config").map(|s| s.as_str()) {
        Some(config_path) => match Config::from_file(config_path) {
            Ok(config) => config,
            Err(msg) => {
                eprintln!("Failed to parse config file {}: {}", config_path, msg);
                std::process::exit(1);
            }
        },
        None => {
            warn!("No config file specified, use default");
            std::process::exit(1);
        }
    };
    conf
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Config {
    pub common: Common,
}

impl Config {
    /// Read configuration from a file into a new Config struct.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let path = path.as_ref();
        debug!("Reading configuration from {}", path.display());

        let data = match fs::read_to_string(path) {
            Ok(data) => data,
            Err(err) => return Err(err.to_string()),
        };

        let conf: Config = match toml::from_str(&data) {
            Ok(conf) => conf,
            Err(err) => return Err(err.to_string()),
        };

        Ok(conf)
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct MongoConfig {
    pub uri: String,
    pub db: String,
}

impl Default for MongoConfig {
    fn default() -> Self {
        Self {
            uri: "mongodb://localhost:27017".to_owned(),
            db: "".to_owned(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct MQConfig {
    pub uri: String,
    pub exchange: String,
    pub routing_key: String,
}

impl Default for MQConfig {
    fn default() -> Self {
        Self {
            uri: "amqp://admin:admin@localhost:5672/".to_owned(),
            exchange: "".to_owned(),
            routing_key: "".to_owned(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct RedisConfig {
    pub uri: String,
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            uri: "localhost:6379".to_owned(),
        }
    }
}


#[derive(Clone, Debug, Deserialize, Default)]
#[serde(default)]
pub struct Common {
    pub log_level: String,
    pub account: String,
    pub password: String,
    pub broker: String,
    pub wsuri: String,
    pub eventmq_ip: String,
    pub database_ip: String,
    pub ping_gap: i32,
    pub taskid: String,
    pub portfolio: String,
    pub bank_password: String,
    pub capital_password: String,
    pub appid: String,
}


pub fn new_config() -> Config {
    let matches = Command::new("QATrader")
        .version("1.0")
        .author("junefar")
        .about("Does awesome things")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("conf\\boot.toml")
                .help("toml文件获取配置")
                .num_args(1),
        )
        .arg(
            Arg::new("account")
                .long("account")
                .help("Set account name")
                .num_args(1),
        )
        .arg(
            Arg::new("password")
                .long("password")
                .help("Set password")
                .num_args(1),
        )
        .arg(
            Arg::new("wsuri")
                .long("wsuri")
                .value_name("ws://localhost:7988")
                .help("Set websocket uri")
                .num_args(1),
        )
        .arg(
            Arg::new("broker")
                .long("broker")
                .value_name("simnow")
                .help("Set broker")
                .num_args(1),
        )
        .arg(
            Arg::new("eventmq_ip")
                .long("eventmq_ip")
                .value_name("amqp://admin:admin@192.168.2.125:5672/")
                .help("接收发单MQ")
                .num_args(1),
        )
        .arg(
            Arg::new("database_ip")
                .long("database_ip")
                .value_name("mongodb://localhost:27017")
                .help("QIFI 数据库")
                .num_args(1),
        )
        .arg(
            Arg::new("ping_gap")
                .long("ping_gap")
                .value_name("5")
                .help("ping 间隔")
                .num_args(1),
        )
        .arg(
            Arg::new("taskid")
                .long("taskid")
                .help("Set taskid")
                .num_args(1),
        )
        .arg(
            Arg::new("portfolio")
                .long("portfolio")
                .value_name("default")
                .help("Set portfolio")
                .num_args(1),
        )
        .arg(
            Arg::new("bank_password")
                .long("bank_password")
                .help("银行密码")
                .num_args(1),
        )
        .arg(
            Arg::new("capital_password")
                .long("capital_password")
                .help("资金密码")
                .num_args(1),
        )
        .arg(
            Arg::new("appid")
                .long("appid")
                .help("Set app id")
                .num_args(1),
        )
        .arg(
            Arg::new("log_level")
                .long("log_level")
                .value_name("info")
                .help("日志等级[ debug / info / warn / error]")
                .num_args(1),
        )
        .get_matches();
    let _args: Vec<String> = env::args().collect();
    if _args.len() <= 1{
        println!("Help:\ncargo build --release\ntarget\\release\\qatrader --help");
        std::process::exit(0);
    }
    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let get = |key: &str| matches.get_one::<String>(key).map(|s| s.as_str());

    if let Some(config_path) = get("config") {
        match Config::from_file(config_path) {
            Ok(config) => config,
            Err(msg) => {
                eprintln!("Failed to parse config file {}: {}", config_path, msg);
                std::process::exit(1);
            }
        }
    } else {
        let account = get("account").unwrap_or("").to_string();
        let password = get("password").unwrap_or("").to_string();
        let wsuri = get("wsuri").unwrap_or("ws://localhost:7988").to_string();
        let broker = get("broker").unwrap_or("simnow").to_string();
        let eventmq_ip = get("eventmq_ip").unwrap_or("").to_string();
        let database_ip = get("database_ip").unwrap_or("").to_string();
        let ping_gap = get("ping_gap")
            .unwrap_or("5")
            .parse::<i32>()
            .unwrap_or_else(|_| {
                eprintln!("Invalid ping_gap value, expected integer");
                std::process::exit(1);
            });
        let taskid = get("taskid").unwrap_or("").to_string();
        let portfolio = get("portfolio").unwrap_or("default").to_string();
        let bank_password = get("bank_password").unwrap_or("").to_string();
        let capital_password = get("capital_password").unwrap_or("").to_string();
        let appid = get("appid").unwrap_or("").to_string();
        let log_level = get("log_level").unwrap_or("info").to_string();
        Config {
            common: Common {
                account,
                password,
                broker,
                wsuri,
                eventmq_ip,
                database_ip,
                ping_gap,
                taskid,
                portfolio,
                bank_password,
                capital_password,
                appid,
                log_level,
            }
        }
    }
}

lazy_static! {
    pub static ref CONFIG: Config = new_config();
}
