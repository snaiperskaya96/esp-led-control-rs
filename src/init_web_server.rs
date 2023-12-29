use embedded_svc::{http::Headers, io::{Write, Read}};
use esp_idf_svc::http::server::EspHttpServer;

use crate::wifi::{WifiCredentials, set_wifi_creds};

pub fn start_web_server() -> anyhow::Result<EspHttpServer<'static>> {
    let web_server_configuartion = esp_idf_svc::http::server::Configuration {
        ..Default::default()
    };

    let mut server = EspHttpServer::new(&web_server_configuartion)?;

    server.fn_handler("/", esp_idf_svc::http::Method::Get, |req| {
        let mut res = req.into_ok_response()?;
        res.write(include_str!("../http/index.html").as_bytes())?;
        Ok(())
    })?;

    server.fn_handler("/store_wifi_info", esp_idf_svc::http::Method::Post, |mut req| {
        let len = req.content_len().unwrap_or(0) as usize;

        log::info!("/store_wifi_info got {} bytes", len);

        if len > 512 {
            log::info!("Request too large. Aborting");
            req.into_status_response(413)?
                .write_all("Request too big".as_bytes())?;
            return Ok(());
        }

        let mut buf = vec![0; len];
        req.read_exact(&mut buf)?;
        let mut resp = req.into_ok_response()?;

        match serde_json::from_slice::<WifiCredentials>(&buf) {
            Ok(creds) => {
                log::info!("Parsed wifi config: {:?}", creds);
                set_wifi_creds(creds)?;
                write!(
                    resp,
                    "Credentials saved. The device will now restart."
                )?;
    
                resp.flush()?;

                log::info!("Restarting in a second.");
                std::thread::sleep(std::time::Duration::from_secs(1));

                esp_idf_hal::reset::restart();
            },
            Err(err) => {
                log::info!("There was an error while parsing user json: {:?}.", err);
                resp.write_all(format!("JSON error: {:?}", err).as_bytes())?;
            },
        }
        Ok(())
    })?;

    log::info!("Web server started.");

    Ok(server)
}

