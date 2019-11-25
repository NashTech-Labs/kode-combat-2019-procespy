extern crate job_scheduler;

use job_scheduler::{Job, JobScheduler};
use procespy::notification::{display_popup, send_email};
use procespy::process_manager::{get_process_details, kill_process};
use procespy::configuration::fetch_configuration;
use procespy::validation::validate_threshold;
use log::{error, info};
use std::collections::HashMap;
use std::collections::hash_map::RandomState;

static CONFIG_FILE: &str = "config.txt";
static CONFIG_ERROR: &str = "Unable to fetch configuration";
static INTELLIJ_THRESHOLD: &str = "INTELLIJ_THRESHOLD";
static EMAIL_ADDRESS: &str = "EMAIL_ADDRESS";
static INTELLIJ: &str = "intellij";
static POPUP_COMMAND: &str = "zenity";
static SCHEDULE_DURATION: &str = "SCHEDULE_DURATION";
static RUST_LOG: &str = "RUST_LOG";
static LOG_LEVEL: &str = "procespy=info";

#[cfg_attr(tarpaulin, skip)]
fn main() {
    std::env::set_var(RUST_LOG, LOG_LEVEL);
    env_logger::init();
    let configs: HashMap<String, String, RandomState> = fetch_configuration(CONFIG_FILE).expect(
        CONFIG_ERROR);
    let intellij_threshold: &str = configs.get(INTELLIJ_THRESHOLD).unwrap().as_str();
    let email_address: &str = configs.get(EMAIL_ADDRESS).unwrap().as_str();
    let timer: String = configs.get(SCHEDULE_DURATION).unwrap().as_str().to_string();
    info!("Processing!!!");
    let mut scheduler: JobScheduler = JobScheduler::new();
    scheduler.add(Job::new(timer.parse().unwrap(), || {
        let details = get_process_details(INTELLIJ);
        let process_details: Vec<&str> = details.as_str().splitn(6, ' ').collect();
        match validate_threshold(process_details[3], intellij_threshold.parse::<f64>().unwrap()) {
            Ok(response) => {
                if response {
                    info!("{}", kill_process(process_details[2]));
                    info!("{}", send_email(email_address, INTELLIJ));
                    info!("{}", display_popup(INTELLIJ, POPUP_COMMAND));
                }
            }
            Err(error) => error!("{}", error)
        }
    }));
    loop {
        scheduler.tick();
    }
}
