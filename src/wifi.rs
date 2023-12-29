use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use serde::{Deserialize, Serialize};

use crate::nvs::get_nvs;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct WifiCredentials {
    ssid: String,
    password: String,
}

pub fn connect_wifi(wifi: &mut BlockingWifi<EspWifi<'static>>, wifi_creds: WifiCredentials) -> anyhow::Result<()> {
    let wifi_configuration: embedded_svc::wifi::Configuration = embedded_svc::wifi::Configuration::Client(embedded_svc::wifi::ClientConfiguration {
        ssid: wifi_creds.ssid.as_str().into(),
        password: wifi_creds.password.as_str().into(),
        ..Default::default()
    });

    wifi.set_configuration(&wifi_configuration)?;

    wifi.start()?;
    log::info!("Wifi started");

    wifi.connect()?;
    log::info!("Wifi connected");

    wifi.wait_netif_up()?;
    log::info!("Wifi netif up");

    Ok(())
}

pub fn get_wifi_creds() -> anyhow::Result<WifiCredentials> {
    let nvs = get_nvs()?;

    let len = nvs.get_u16("wifi-creds-len")?.unwrap_or(0);

    let mut cred_buf = [0_u8; 512];
    nvs.get_raw("wifi-creds", &mut cred_buf)?;

    let trimmed_data = &cred_buf.to_vec()[0..len as _];

    log::info!("Got wifi credentials blob from NVS: {:?}", String::from_utf8(trimmed_data.to_vec()));

    let creds: WifiCredentials = serde_json::from_slice(trimmed_data)?;

    Ok(creds)
}

pub fn set_wifi_creds(creds: WifiCredentials) -> anyhow::Result<()> {
    let mut nvs = get_nvs()?;

    let json: String = serde_json::to_string(&creds)?;

    nvs.set_u16("wifi-creds-len", json.len() as _)?;
    nvs.set_raw("wifi-creds", json.as_bytes())?;

    Ok(())
}