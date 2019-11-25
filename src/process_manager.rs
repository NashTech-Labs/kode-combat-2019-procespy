use std::process::Command;
use subprocess::Exec;

pub static PROCESS_COMMAND: &str = "ps -eo pid,%mem,cmd --sort=-%mem";
pub static SUCCESS_MESSAGE: &str = "Successfully killed process.";
pub static ERROR_MESSAGE: &str = "Unable to kill process.";
pub static PROCESS_ERROR: &str = "Given process not running on the system.";
pub static KILL: &str = "kill";
static FILTER_COMMAND: &str = "grep -m1 ";

pub fn get_process_details(process_name: &str) -> String {
    let mut filter_command: String = FILTER_COMMAND.to_string();
    filter_command.push_str(process_name);
    match {
        Exec::shell(PROCESS_COMMAND) | Exec::shell(filter_command)
    }.capture() {
        Ok(data) => data.stdout_str(),
        Err(_error) => PROCESS_ERROR.to_string()
    }
}

pub fn kill_process(process_id: &str) -> &str {
    match Command::new(KILL)
        .arg(process_id).output() {
        Ok(_) => SUCCESS_MESSAGE,
        Err(_) => ERROR_MESSAGE
    }
}

#[cfg(test)]
mod test {
    use crate::process_manager::{get_process_details, SUCCESS_MESSAGE, kill_process};

    static INTELLIJ: &str = "intellij";
    static PROCESS_ID: &str = "123";

    #[test]
    fn test_get_process_details() {
        assert!(get_process_details(INTELLIJ).contains(" "));
    }

    #[test]
    fn test_kill_process_success() {
        assert_eq!(kill_process(PROCESS_ID), SUCCESS_MESSAGE);
    }
}