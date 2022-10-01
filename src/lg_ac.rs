use core::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Mode {
    OFF,
    FAN,
    AI,
    AC,
    DEHUM,
    HEAT,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::OFF => write!(f, "OFF"),
            Mode::FAN => write!(f, "FAN"),
            Mode::AI => write!(f, "AI"),
            Mode::AC => write!(f, "AC"),
            Mode::DEHUM => write!(f, "DEHUM"),
            Mode::HEAT => write!(f, "HEAT"),
        }
    }
}

impl FromStr for Mode {
    type Err = ();

    fn from_str(mode: &str) -> Result<Mode, Self::Err> {
        match mode {
            "OFF" => Ok(Mode::OFF),
            "FAN" => Ok(Mode::FAN),
            "AI" => Ok(Mode::AI),
            "AC" => Ok(Mode::AC),
            "DEHUM" => Ok(Mode::DEHUM),
            "HEAT" => Ok(Mode::HEAT),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone, Serialize, Deserialize)]
pub enum FanMode {
    LOW,
    MEDIUM,
    HIGH,
    CHAOS,
}

impl FromStr for FanMode {
    type Err = ();

    fn from_str(mode: &str) -> Result<FanMode, Self::Err> {
        match mode {
            "LOW" => Ok(FanMode::LOW),
            "MEDIUM" => Ok(FanMode::MEDIUM),
            "HIGH" => Ok(FanMode::HIGH),
            "CHAOS" => Ok(FanMode::CHAOS),
            _ => Err(()),
        }
    }
}

impl fmt::Display for FanMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FanMode::LOW => write!(f, "LOW"),
            FanMode::MEDIUM => write!(f, "MEDIUM"),
            FanMode::HIGH => write!(f, "HIGH"),
            FanMode::CHAOS => write!(f, "CHAOS"),
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct State {
    pub updated: bool,
    pub mode: Mode,
    pub min_temp: i32,
    pub max_temp: i32,
    pub target_temp: f64,
    pub fan_speed: i32,
    pub fan_mode: FanMode,
}

impl State {
    pub fn from_lirc_command(command: &str) -> Result<State, &str> {
        let mut s: State = State::default();
        let cmd = command.split(' ').collect::<Vec<&str>>()[2];
        let parts_count = cmd.split('_').count();
        let mut parts = cmd.split('_');

        let mode = parts.next().expect("Missing mode part of command");
        s.mode = Mode::from_str(mode).expect("Failed to parse mode");

        let part = parts.next().expect("Missing fan mode part of command");
        s.fan_mode = FanMode::from_str(part).expect("Failed to parse fan mode");

        if parts_count > 2 {
            let temp = parts.next().expect("Command missing temperature");
            s.target_temp = temp.parse().expect("Expected a number");
        }

        Ok(s)
    }

    pub fn from_state(state: State) -> String {
        println!("Sending new state on IR: {:?}", state);

        let mut cmd = String::from("");

        cmd += &state.mode.to_string();
        cmd += "_";
        cmd += &state.fan_mode.to_string();
        cmd += "_";
        cmd += state.target_temp.to_string().as_str();
        println!("{cmd}");
        cmd
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            updated: true,
            mode: Mode::OFF,
            min_temp: 18,
            max_temp: 30,
            target_temp: 18.0,
            fan_speed: 0,
            fan_mode: FanMode::LOW,
        }
    }
}

#[test]
fn from_lirc_command_test() {
    let s = State::from_lirc_command("0000000000000028 00 AC_HIGH_21 LG_AC");
    assert!(s.is_ok());
    let state = s.unwrap();
    assert_eq!(state.mode, Mode::AC);
    assert_eq!(state.min_temp, 18);
    assert_eq!(state.max_temp, 30);
    assert_eq!(state.target_temp, 21.0);
    assert_eq!(state.fan_mode, FanMode::HIGH);
}

#[test]
fn from_state_test() {
    let s = State {
        updated: true,
        mode: Mode::AC,
        min_temp: 18,
        max_temp: 30,
        target_temp: 21.0,
        fan_speed: 0,
        fan_mode: FanMode::HIGH,
    };

    let cmd = State::from_state(s);

    assert_eq!(cmd, "AC_HIGH_21");
}
