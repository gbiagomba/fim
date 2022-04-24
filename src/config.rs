// Copyright (C) 2021, Achiefs.

// Global constants definitions
pub const VERSION: &str = "0.2.2";
pub const NETWORK_MODE: &str = "NETWORK";
pub const FILE_MODE: &str = "FILE";
pub const BOTH_MODE: &str = "BOTH";
const CONFIG_LINUX_PATH: &str = "/etc/fim/config.yml";

// To parse files in yaml format
use yaml_rust::yaml::{Yaml, YamlLoader, Array};
// To use files IO operations.
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::io::Write;
// To manage paths
use std::path::Path;
use std::env;
// To set log filter level
use simplelog::LevelFilter;

// ----------------------------------------------------------------------------

pub struct Config {
    pub version: String,
    pub path: String,
    pub events_destination: String,
    pub endpoint_address: String,
    pub endpoint_user: String,
    pub endpoint_pass: String,
    pub events_file: String,
    pub monitor: Array,
    pub nodename: String,
    pub log_file: String,
    pub log_level: String,
    pub system: String
}

impl Config {

    pub fn new() -> Self {
        println!("[INFO] System detected {}", env::consts::OS);
        // Select directory where to load config.yml it depends on system
        let default_path = format!("./config/{}/config.yml", env::consts::OS);
        let config_path = match Path::new(default_path.as_str()).exists() {
            true => String::from(default_path.as_str()),
            false => String::from(CONFIG_LINUX_PATH)
        };
        println!("[INFO] Loaded config from: {}", config_path);
        let yaml = read_config(config_path.clone());

        // Manage null value on events->destination value
        let events_destination = match yaml[0]["events"]["destination"].as_str() {
            Some(value) => String::from(value),
            None => {
                println!("[WARN] events->destination not found in config.yml, using 'file'.");
                String::from("file")
            }
        };

        // Manage null value on events->file value
        let events_file = match yaml[0]["events"]["file"].as_str() {
            Some(value) => String::from(value),
            None => {
                if events_destination != *"network" {
                    println!("[ERROR] events->file not found in config.yml.");
                    panic!("events->file not found in config.yml.");
                }else{
                    String::from("Not_used")
                }
            }
        };

        // Manage null value on events->endpoint->address value
        let endpoint_address = match yaml[0]["events"]["endpoint"]["address"].as_str() {
            Some(value) => String::from(value),
            None => {
                if events_destination != *"file" {
                    println!("[ERROR] events->endpoint->address not found in config.yml.");
                    panic!("events->endpoint->address not found in config.yml.");
                }else{
                    String::from("Not_used")
                }
            }
        };

        // Manage null value on events->endpoint->credentials->user value
        let endpoint_user = match yaml[0]["events"]["endpoint"]["credentials"]["user"].as_str() {
            Some(value) => String::from(value),
            None => {
                if events_destination != *"file" {
                    println!("[ERROR] events->endpoint->credentials->user not found in config.yml.");
                    panic!("events->endpoint->credentials->user not found in config.yml.");
                }else{
                    String::from("Not_used")
                }
            }
        };

        // Manage null value on events->endpoint->credentials->password value
        let endpoint_pass = match yaml[0]["events"]["endpoint"]["credentials"]["password"].as_str() {
            Some(value) => String::from(value),
            None => {
                if events_destination != *"file" {
                    println!("[ERROR] events->endpoint->credentials->password not found in config.yml.");
                    panic!("events->endpoint->credentials->password not found in config.yml.");
                }else{
                    String::from("Not_used")
                }
            }
        };

        // Manage null value on monitor value
        let monitor = match yaml[0]["monitor"].as_vec() {
            Some(value) => value.to_vec(),
            None => {
                println!("[ERROR] monitor not found in config.yml.");
                panic!("monitor not found in config.yml.");
            }
        };

        // Manage null value on nodename value
        let nodename = match yaml[0]["nodename"].as_str() {
            Some(value) => String::from(value),
            None => {
                println!("[WARN] nodename not found in config.yml, using 'FIM'.");
                String::from("FIM")
            }
        };

        // Manage null value on log->file value
        let log_file = match yaml[0]["log"]["file"].as_str() {
            Some(value) => String::from(value),
            None => {
                println!("[ERROR] log->file not found in config.yml.");
                panic!("log->file not found in config.yml.");
            }
        };

        // Manage null value on log->level value
        let log_level = match yaml[0]["log"]["level"].as_str() {
            Some(value) => String::from(value),
            None => {
                println!("[WARN] log->level not found in config.yml, using 'info'.");
                String::from("info")
            }
        };

        Config {
            version: String::from(VERSION),
            path: config_path,
            events_destination,
            endpoint_address,
            endpoint_user,
            endpoint_pass,
            events_file,
            monitor,
            nodename,
            log_file,
            log_level,
            system: String::from(env::consts::OS)
        }
    }

    // ------------------------------------------------------------------------

    // To process log level set on config file
    pub fn get_level_filter(&self) -> LevelFilter {
        let mut log = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(self.log_file.clone())
            .expect("Unable to open events log file.");

        match self.log_level.as_str() {
            "debug" | "Debug" | "DEBUG" | "D" | "d" => LevelFilter::Debug,
            "info" | "Info" | "INFO" | "I" | "i" => LevelFilter::Info,
            "error" | "Error" | "ERROR" | "E" | "e" => LevelFilter::Error,
            "warning" | "Warning" | "WARNING" | "W" | "w" | "warn" | "Warn" | "WARN" => LevelFilter::Warn,
            _ => {
                let msg = String::from("[ERROR] invalid log level from 'config.yml', using Info level.");
                println!("{}", msg);
                writeln!(log, "{}", msg).expect("[ERROR] cannot write in log file.");
                LevelFilter::Info
            }
        }
    }

    // ------------------------------------------------------------------------

    pub fn get_events_destination(&self) -> String {
        match self.events_destination.clone().as_str() {
            "both" => String::from(BOTH_MODE),
            "network" => String::from(NETWORK_MODE),
            // Default option is to log into file
            _ => String::from(FILE_MODE)
        }
    }

}

// ----------------------------------------------------------------------------

// To read the Yaml configuration file
pub fn read_config(path: String) -> Vec<Yaml> {
    let mut file = File::open(path).expect("Unable to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Unable to read file");
    YamlLoader::load_from_str(&contents).unwrap()
}


// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // ------------------------------------------------------------------------

    fn create_test_config(filter: &str) -> Config {
        Config {
            version: String::from(VERSION),
            path: String::from("test"),
            events_destination: String::from("test"),
            endpoint_address: String::from("test"),
            endpoint_user: String::from("test"),
            endpoint_pass: String::from("test"),
            events_file: String::from("test"),
            monitor: Array::new(),
            nodename: String::from("test"),
            log_file: String::from("./test.log"),
            log_level: String::from(filter),
            system: String::from("test")
        }
    }

    // ------------------------------------------------------------------------

    #[test]
    fn test_get_level_filter() {
        let filter = LevelFilter::Info;
        assert_eq!(create_test_config("info").get_level_filter(), filter);
        assert_eq!(create_test_config("Info").get_level_filter(), filter);
        assert_eq!(create_test_config("INFO").get_level_filter(), filter);
        assert_eq!(create_test_config("I").get_level_filter(), filter);
        assert_eq!(create_test_config("i").get_level_filter(), filter);
    }

    // ------------------------------------------------------------------------

    #[test]
    fn test_get_level_filter_debug() {
        let filter = LevelFilter::Debug;
        assert_eq!(create_test_config("debug").get_level_filter(), filter);
        assert_eq!(create_test_config("Debug").get_level_filter(), filter);
        assert_eq!(create_test_config("DEBUG").get_level_filter(), filter);
        assert_eq!(create_test_config("D").get_level_filter(), filter);
        assert_eq!(create_test_config("d").get_level_filter(), filter);
    }

    // ------------------------------------------------------------------------

    #[test]
    fn test_get_level_filter_error() {
        let filter = LevelFilter::Error;
        assert_eq!(create_test_config("error").get_level_filter(), filter);
        assert_eq!(create_test_config("Error").get_level_filter(), filter);
        assert_eq!(create_test_config("ERROR").get_level_filter(), filter);
        assert_eq!(create_test_config("E").get_level_filter(), filter);
        assert_eq!(create_test_config("e").get_level_filter(), filter);
    }

    // ------------------------------------------------------------------------

    #[test]
    fn test_get_level_filter_warning() {
        let filter = LevelFilter::Warn;
        assert_eq!(create_test_config("warning").get_level_filter(), filter);
        assert_eq!(create_test_config("Warning").get_level_filter(), filter);
        assert_eq!(create_test_config("WARNING").get_level_filter(), filter);
        assert_eq!(create_test_config("W").get_level_filter(), filter);
        assert_eq!(create_test_config("w").get_level_filter(), filter);
        assert_eq!(create_test_config("warn").get_level_filter(), filter);
        assert_eq!(create_test_config("Warn").get_level_filter(), filter);
        assert_eq!(create_test_config("WARN").get_level_filter(), filter);
    }

    // ------------------------------------------------------------------------

    #[test]
    fn test_get_level_filter_bad() {
        let filter = LevelFilter::Info;
        assert_eq!(create_test_config("bad").get_level_filter(), filter);
        assert_eq!(create_test_config("BAD").get_level_filter(), filter);
        assert_eq!(create_test_config("B").get_level_filter(), filter);
        assert_eq!(create_test_config("b").get_level_filter(), filter);
        assert_eq!(create_test_config("test").get_level_filter(), filter);
        assert_eq!(create_test_config("anything").get_level_filter(), filter);
        assert_eq!(create_test_config("").get_level_filter(), filter);
        assert_eq!(create_test_config("_").get_level_filter(), filter);
        assert_eq!(create_test_config("?").get_level_filter(), filter);
        assert_eq!(create_test_config("=").get_level_filter(), filter);
        assert_eq!(create_test_config("/").get_level_filter(), filter);
        assert_eq!(create_test_config(".").get_level_filter(), filter);
        assert_eq!(create_test_config(":").get_level_filter(), filter);
        assert_eq!(create_test_config(";").get_level_filter(), filter);
        assert_eq!(create_test_config("!").get_level_filter(), filter);
        assert_eq!(create_test_config("''").get_level_filter(), filter);
        assert_eq!(create_test_config("[]").get_level_filter(), filter);
    }

    // ------------------------------------------------------------------------

    #[test]
    fn test_get_level_filter_empty() {
        assert_eq!(create_test_config("").get_level_filter(), LevelFilter::Info);
    }

    // ------------------------------------------------------------------------

    #[test]
    fn test_read_config() {
        read_config(String::from("config/linux/config.yml"));
    }

    // ------------------------------------------------------------------------

    #[test]
    #[should_panic(expected = "NotFound")]
    fn test_read_config_panic() {
        read_config(String::from("not_found"));
    }

    // ------------------------------------------------------------------------

    #[test]
    #[should_panic(expected = "ScanError")]
    fn test_read_config_panic_not_config() {
        read_config(String::from("README.md"));
    }
}
