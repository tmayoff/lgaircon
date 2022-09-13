// https://github.com/awendland/rpi-ds18b20-rust

use std::{fs,io,num};
use std::path::PathBuf;

static W1_PATH_PREFIX: &str = "/sys/bus/w1/devices";
static W1_PATH_SUFFIX: &str = "w1_slave";

#[derive(Debug)]
pub enum W1Error {
    Io(io::Error),
    Parse(num::ParseIntError),
    BadSerialConnection,
}

impl From<io::Error> for W1Error {
    fn from(err: io::Error) -> W1Error {
        W1Error::Io(err)
    }
}

impl From<num::ParseIntError> for W1Error {
    fn from(err: num::ParseIntError) -> W1Error {
        W1Error::Parse(err)
    }
}

pub struct MilliCelsius(u32);
impl MilliCelsius {
    pub fn to_celsius(self) -> f64 {
        (self.0 as f64) / 1000.0
    }
}

pub struct DS18B20 {
    w1_id: String
}

impl DS18B20 {
    pub fn new() -> Result<DS18B20, io::Error> {
        for entry in fs::read_dir(W1_PATH_PREFIX)? {
            let filename = entry?.file_name().into_string().unwrap();
            if filename.contains("28-") {
                return Ok(DS18B20 {
                    w1_id: filename
                })
            }
        }

        Err(io::Error::from(io::ErrorKind::NotFound))
    }

    pub fn read_raw(&self) -> io::Result<String> {
        let mut path = PathBuf::from(W1_PATH_PREFIX);
        path.push(&self.w1_id);
        path.push(W1_PATH_SUFFIX);
        fs::read_to_string(path)
    }

    pub fn read_temp(&self) -> Result<MilliCelsius, W1Error> {
        let temp_data = self.read_raw()?;
        if !temp_data.contains("YES") {
            return Err(W1Error::BadSerialConnection);
        }
        Ok(MilliCelsius(parse_temp(temp_data)?))
    }
}

fn parse_temp(temp_data: String) -> Result<u32, num::ParseIntError> {
    let (_, temp_str) = temp_data.split_at(temp_data.find("t=").unwrap() + 2);
    temp_str.trim().parse::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_temp() {
        let temp_data ="6e 01 55 05 7f 7e a5 66 f2 : crc=f2 YES
6e 01 55 05 7f 7e a5 66 f2 t=22875".to_string();
        assert_eq!(Ok(22875), parse_temp(temp_data));
    }
}