use esp_idf_svc::nvs::{EspNvs, NvsDefault, EspNvsPartition, EspDefaultNvsPartition};


lazy_static::lazy_static! {
    pub static ref NVS_DEFAULT_PARTITION: EspNvsPartition<NvsDefault> = {
        EspDefaultNvsPartition::take().unwrap()
    };
}

pub fn get_nvs() -> anyhow::Result<EspNvs<NvsDefault>> {
    let namespace = "wifi";
    let nvs = match EspNvs::new(NVS_DEFAULT_PARTITION.clone(), namespace, true) {
        Ok(nvs) => {
            log::info!("Got namespace {:?} from default partition", namespace);
            nvs
        }
        Err(e) => { log::info!("Couldn't get NVS namespace {:?}", e); return Err(e.into()); },
    };

    Ok(nvs)
}
