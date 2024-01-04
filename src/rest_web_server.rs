use embedded_svc::{http::Headers, io::{Read, Write}};
use esp_idf_svc::http::server::{EspHttpServer, EspHttpConnection};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct StateRequest {
    alias: String,
    dim: Option<u8>,
}

fn get_state_from_request<'a>(req: &mut embedded_svc::http::server::Request<&mut EspHttpConnection<'a>>) -> anyhow::Result<StateRequest> 
{
    let len = req.content_len().unwrap_or(0) as usize;

    if len > 512 {
        return Err(anyhow::anyhow!("Request is too large"));
    }

    let mut buf = vec![0; len];
    req.read_exact(&mut buf)?;

    match serde_json::from_slice::<StateRequest>(&buf) {
        Ok(state) => Ok(state),
        Err(e) => Err(anyhow::anyhow!(e)),
    }
}

pub fn start_rest_server() -> anyhow::Result<EspHttpServer<'static>> {
    let web_server_configuartion = esp_idf_svc::http::server::Configuration {
        ..Default::default()
    };

    let mut server = EspHttpServer::new(&web_server_configuartion)?;

    server.fn_handler("/turn_on", esp_idf_svc::http::Method::Post, |mut req| {
        let state = get_state_from_request(&mut req)?;

        let p = crate::pwm::gpio_driver(&state.alias)?;
        log::info!("Turning on gpio {}.", state.alias);
        p.lock().unwrap().enable()?;

        let mut res = req.into_ok_response()?;
        res.flush()?;
        Ok(())
    })?;
    
    server.fn_handler("/turn_off", esp_idf_svc::http::Method::Post, |mut req| {
        let state = get_state_from_request(&mut req)?;
       
        let p = crate::pwm::gpio_driver(&state.alias)?;
        log::info!("Turning off gpio {}.", state.alias);

        p.lock().unwrap().disable()?;

        let mut res = req.into_ok_response()?;
        res.flush()?;
        Ok(())
    })?;

    server.fn_handler("/dim", esp_idf_svc::http::Method::Post, |mut req| {
        let state = get_state_from_request(&mut req)?;
        
        let p = crate::pwm::gpio_driver(&state.alias)?;
        let mut gpio = p.lock().unwrap();
        let duty = state.dim.unwrap() as f32 * gpio.get_max_duty() as f32 / 100_f32;

        log::info!("Setting gpio {} duty to {}", state.alias, duty as u32);
        gpio.set_duty(duty as _)?;

        let mut res = req.into_ok_response()?;
        res.flush()?;
        Ok(())
    })?;

    // /status?alias=light_1
    server.fn_handler("/status", esp_idf_svc::http::Method::Get, |req| {
        let uri = req.uri().to_string();
        let uri = uri.replace("/status", "");
        let query_params = querystring::querify(uri.as_str());

        match query_params.iter().find(|x| x.0 == "alias") {
            Some(alias) => {
                let p = crate::pwm::gpio_driver(&alias.1.to_string())?;

                let gpio = p.lock().unwrap();
                let duty = (gpio.get_duty() as f32) / (gpio.get_max_duty() as f32) * 100_f32;        
        
                #[derive(Serialize)]
                struct StatusResp {
                    is_active: bool,
                    duty: f32,
                }
        
                let status = StatusResp { is_active: gpio.get_duty() != 0, duty };
        
                let mut res = req.into_ok_response()?;
                write!(res, "{}", serde_json::to_string(&status)?)?;
            },
            None => {
                let mut res = req.into_status_response(404)?;
                write!(res, "Should specify an alias.")?;
            },
        }

        Ok(())
    })?;
    
    log::info!("REST server started.");
    Ok(server)
}

