use dependencies_sync::toml;
use serde::Serialize;
use serde_derive::Deserialize;

use crate::configs_map::get_configs_map;

pub trait ConfigTrait {
    fn register_config(config_name: &str, config: impl Serialize + Deserialize,){
        let map_arc = get_configs_map();
        let map = map_arc.write();

        let toml_v = toml::Value::from(config);

        map.insert(config_name, toml_v);
    }

    fn get_config(config_name: &str) -> Option<impl Deserialize>{
        let map_arc = get_configs_map();
        let map = map_arc.read();

        let toml_v = map.get(config_name)?;

        let config = toml::from_str(toml_v.as_str().unwrap()).unwrap();

        Some(config)
    }
}