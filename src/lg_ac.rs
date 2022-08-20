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
    High
}

pub struct State {
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
        let mut parts = cmd.split('_');
        
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
}

impl Default for State {
    fn default() -> Self {
        State {
            mode: Mode::Cool,
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