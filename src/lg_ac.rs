#[derive(PartialEq, Debug)]
enum Mode {
    Fan,
    Cool,
    Heat,
    Dehumidifier
}

#[derive(PartialEq, Debug)]
enum FanMode {
    Number,
    Low,
    Medium,
    High,
    Chaos
}

pub struct State {
    on: bool,
    mode: Mode,
    min_temp: i32,
    max_temp: i32,
    cur_temp: i32,
    fan_speed: i32,
    fan_mode: FanMode,
}

impl State {
    pub fn from_lirc_command(command: &str) -> Result<State, &str> {

        let mut s: State = State::default();
        let cmd = command.split(' ').collect::<Vec<&str>>()[2];
        let parts_count = cmd.split('_').count();
        let mut parts = cmd.split('_');

        if parts_count == 2 {
            let mut s: State = Default::default();
            parts.next();
            let on = parts.next().unwrap();
            match on {
                "ON" => s.on = true,
                "OFF" => s.on = false,
                _ => return Err("Failed to parse ON/OFF state"),
            }

            return Ok(s);
        }

        // get mode
        let mode = parts.next().unwrap();
        match mode {
            "AC" => s.mode = Mode::Cool,
            "HEAT" => s.mode = Mode::Heat,
            "DEHUM" => s.mode = Mode::Dehumidifier,
            _ => return Err("Failed to parse mode"),
        }

        // get fan speed
        let fanspeed = parts.next().unwrap();
        match fanspeed {
            "LOW" => s.fan_mode = FanMode::Low,
            "MED" => s.fan_mode = FanMode::Medium,
            "HIGH" => s.fan_mode = FanMode::High,
            "CHAOS" => s.fan_mode = FanMode::Chaos,
            &_ => {
                let f:i32 = fanspeed.parse().expect("Expected a number");
                s.fan_mode = FanMode::Number;
                s.fan_speed = f;
            }
        }

        let temp = parts.next().unwrap().parse().expect("Expected a number");
        s.cur_temp = temp;

        Ok(s)
    }

    pub fn from_state(state: State) -> String {
        let mut cmd = String::from("");

        match state.mode {
            Mode::Fan => cmd += "FAN",
            Mode::Cool => cmd += "AC",
            Mode::Heat => cmd += "HEAT",
            Mode::Dehumidifier => cmd += "DEHUM"
        }

        cmd += "_";

        match state.fan_mode {
            FanMode::Number => cmd += state.fan_speed.to_string().as_str(),
            FanMode::Low => cmd += "LOW",
            FanMode::Medium => cmd += "MED",
            FanMode::High => cmd += "HIGH",
            FanMode::Chaos => cmd += "CHAOS",
        }

        cmd += "_";

        cmd += state.cur_temp.to_string().as_str();

        cmd
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            mode: Mode::Cool,
            on: false,
            min_temp: 18,
            max_temp: 30,
            cur_temp: 18,
            fan_speed: 0,
            fan_mode: FanMode::Low,
        }
    }
}

#[test]
fn from_lirc_command_test() {
    let s = State::from_lirc_command("0000000000000028 00 AC_HIGH_21 LG_AC");
    assert_eq!(s.is_err(), false);
    let state = s.unwrap();
    assert_eq!(state.mode, Mode::Cool);
    assert_eq!(state.min_temp, 18);
    assert_eq!(state.max_temp, 30);
    assert_eq!(state.cur_temp, 21);
    assert_eq!(state.fan_mode, FanMode::High);
}

#[test]
fn from_state_test() {
    let s = State {
        mode: Mode::Cool,
        on: true,
        min_temp: 18,
        max_temp: 30,
        cur_temp: 21,
        fan_speed: 0,
        fan_mode: FanMode::High,
    };

    let cmd = State::from_state(s);
    
    assert_eq!(cmd, "AC_HIGH_21");
}