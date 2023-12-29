mod wifi;
mod nvs;
mod init_web_server;
mod rest_web_server;
mod gpio_config;
mod pwm;

use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::{wifi::{BlockingWifi, EspWifi, Configuration, AccessPointConfiguration, AuthMethod}, eventloop::EspSystemEventLoop};
use wifi::{get_wifi_creds, connect_wifi};


fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let sys_loop = EspSystemEventLoop::take()?;

    let peripherals = Peripherals::take()?;

    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs::NVS_DEFAULT_PARTITION.clone()))?,
        sys_loop,
    )?;

    // holding a reference so we dont drop it right after the match
    let _web_server;
    let _rest_server;

    match get_wifi_creds() {
        Ok(creds) => {
            match connect_wifi(&mut wifi, creds) {
                Ok(_) => {
                    log::info!("Successfully connected to wifi. Starting REST server.");
                    _rest_server = rest_web_server::start_rest_server()?;
                },
                Err(_) => {
                    log::info!("Invalid wifi credentials. Starting AP and web server.");

                    start_ap(&mut wifi)?;
                    _web_server = init_web_server::start_web_server()?;
                },
            }
        },
        Err(e) => {
            log::info!("No wifi credentials available ({:?}). Starting AP and web server.", e);

            start_ap(&mut wifi)?;
            _web_server = init_web_server::start_web_server()?;
        },
    }

    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

fn start_ap(wifi: &mut BlockingWifi<EspWifi<'static>>) -> anyhow::Result<()> {
    let wifi_configuration: Configuration = Configuration::AccessPoint(AccessPointConfiguration {
        ssid: "led-control".into(),
        auth_method: AuthMethod::WPA2Personal,
        password: "led-control123".into(),
        channel: 1,
        ..Default::default()
    });

    wifi.set_configuration(&wifi_configuration)?;
    wifi.start()?;

    wifi.wait_netif_up()?;

    log::info!("AP started.");

    Ok(())
}