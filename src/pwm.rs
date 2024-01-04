use std::sync::{Arc, Mutex};

use crate::gpio_config;
use esp_idf_hal::{
    gpio::AnyOutputPin,
    ledc::*,
};
use once_cell::sync::OnceCell;

pub fn timer_driver(
) -> &'static Arc<LedcTimerDriver<'static>> {
    static INSTANCE: OnceCell<Arc<LedcTimerDriver<'static>>> = OnceCell::new();
    INSTANCE.get_or_init(|| {

        let gpio_configs = gpio_config::get_configs().unwrap();

        let highest_frequency = gpio_configs.iter().fold(0, |a, b| a.max(b.1.frequency));
        let timer_config = config::TimerConfig::new().frequency(highest_frequency.into());

        Arc::new(LedcTimerDriver::new(unsafe { TIMER0::new() }, &timer_config).unwrap())
    })
}

pub fn gpio_driver(
    gpio_alias: &String,
) -> anyhow::Result<Arc<Mutex<LedcDriver<'static>>>> {
    lazy_static::lazy_static! {
        static ref GPIO_DRIVERS: Vec<Arc<Mutex<LedcDriver<'static>>>> = {
            let mut drivers = Vec::new();
            let gpio_configs = gpio_config::get_configs().unwrap();
    
            let timer = timer_driver().clone();
            for gpio_config in gpio_configs {
                let pin = gpio_config.1.gpio as _;
                // hacking the channel stuff because i can't really be arsed
                // to look for a "proper" way, assuming there's any.
                // All those drivers taking peripherals do so by taking ownership
                // and it's just terrible to handle that in a (lazy) static environment.
                // No, thanks.
                let driver = match gpio_config.1.channel {
                    0 => {
                        LedcDriver::new(
                            unsafe { CHANNEL0::new() },
                            timer.clone(),
                            unsafe { AnyOutputPin::new(pin) },
                        )
                    },
                    1 => {
                        LedcDriver::new(
                            unsafe { CHANNEL1::new() },
                            timer.clone(),
                            unsafe { AnyOutputPin::new(pin) },
                        )
                    },
                    2 => {
                        LedcDriver::new(
                            unsafe { CHANNEL2::new() },
                            timer.clone(),
                            unsafe { AnyOutputPin::new(pin) },
                        )
                    },
                    3 => {
                        LedcDriver::new(
                            unsafe { CHANNEL3::new() },
                            timer.clone(),
                            unsafe { AnyOutputPin::new(pin) },
                        )
                    },
                    4 => {
                        LedcDriver::new(
                            unsafe { CHANNEL4::new() },
                            timer.clone(),
                            unsafe { AnyOutputPin::new(pin) },
                        )
                    },
                    5 => {
                        LedcDriver::new(
                            unsafe { CHANNEL5::new() },
                            timer.clone(),
                            unsafe { AnyOutputPin::new(pin) },
                        )
                    },
                    6 => {
                        LedcDriver::new(
                            unsafe { CHANNEL6::new() },
                            timer.clone(),
                            unsafe { AnyOutputPin::new(pin) },
                        )
                    },
                    7 => {
                        LedcDriver::new(
                            unsafe { CHANNEL7::new() },
                            timer.clone(),
                            unsafe { AnyOutputPin::new(pin) },
                        )
                    },
                    _ => panic!()
                }.unwrap();
    
                drivers.push(Arc::new(Mutex::new(driver)));
            }
    
            drivers
        };
    };

    let gpio_configs = gpio_config::get_configs().unwrap();
    let target = match gpio_configs.iter().find(|x| *x.1.alias == *gpio_alias) {
        Some(conf) => conf,
        None => return Err(anyhow::anyhow!("Could not find specified config {}", gpio_alias))
    };

    for driver in &*GPIO_DRIVERS {
        if driver.lock().unwrap().channel() == target.1.channel as u32 {
            return Ok(driver.clone());
        }
    }

    Err(anyhow::anyhow!("Could not find pwm driver"))
}
