use std::{fs,io,num};
use std::path::PathBuf;
use crate::w1_errors::*;



static W1_PATH_PREFIX: &str = "/sys/bus/w1/devices";
static W1_PATH_SUFFIX: &str = "w1_slave";



pub struct MilliCelsius(u32);
impl MilliCelsius {
    pub fn to_fahrenheit(self) -> f64 {
        (self.0 as f64) / 1000.0 / 5.0 * 9.0 + 32.0
    }

    pub fn as_u32(self) -> u32 {
        self.0
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
        panic!("Unable to find a DS18B20")
    }

    pub fn new_for_id(id: String) -> DS18B20 {
         DS18B20 {
             w1_id: id
         }
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