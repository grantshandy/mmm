use std::path::PathBuf;

use crate::update_database;
use crate::STATE;

#[cfg(target_arch = "arm")]
const PIN: u8 = 11;

#[cfg(target_arch = "arm")]
use rppal::gpio::Gpio;

#[cfg(target_arch = "x86_64")]
pub unsafe fn fake_turn_pins_off(path: &PathBuf) -> Result<bool, (bool, String)> {
    println!("debug turned off");
    STATE = false;
    update_database(path);

    return Ok(STATE);
}

#[cfg(target_arch = "x86_64")]
pub unsafe fn fake_turn_pins_on(path: &PathBuf) -> Result<bool, (bool, String)> {
    println!("debug turned on");
    STATE = true;
    update_database(path);

    return Ok(STATE);
}

#[cfg(target_arch = "arm")]
pub unsafe fn turn_pins_off(path: &PathBuf) -> Result<bool, (bool, String)> {
    let gpio = match Gpio::new() {
        Ok(gpio) => gpio,
        Err(error) => return Err((STATE, error.to_string())),
    };

    let mut pin = match gpio.get(PIN) {
        Ok(pin) => pin,
        Err(error) => {
            eprintln!("couldn't get the pin!! msg: {}", error);
            return Err((STATE, error.to_string()));
        }
    }
    .into_output();

    pin.set_low();
    println!("turned off");
    STATE = false;
    update_database(path);

    return OK(STATE);
}

#[cfg(target_arch = "arm")]
pub unsafe fn turn_pins_on(path: &PathBuf) -> Result<bool, (bool, String)> {
    let gpio = match Gpio::new() {
        Ok(gpio) => gpio,
        Err(error) => return Err((STATE, error.to_string())),
    };

    let mut pin = match gpio.get(PIN) {
        Ok(pin) => pin,
        Err(error) => {
            eprintln!("couldn't get the pin!! msg: {}", error);
            return Err((STATE, error.to_string()));
        }
    }
    .into_output();

    pin.set_high();
    println!("turned on");
    STATE = true;
    update_database(path);

    return Ok(STATE);
}
