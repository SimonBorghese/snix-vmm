use crate::vm::{CrosVmParam};

pub mod vm;
pub mod snix;

fn match_cfg_key(cmd_line: CrosVmParam, data: &mut json::JsonValue) -> json::JsonValue{
    match cmd_line{
        CrosVmParam::Value(value) => {
            value.into()
        }
        CrosVmParam::Bool(value) => {
            value.into()
        }
        CrosVmParam::Map(map) => {
            let mut sub_object = json::JsonValue::new_object();
            for item in map {
                // Recurse until we find an actual value or bool
                sub_object[item.0] = match_cfg_key(item.1, data);
            }
            sub_object
        }
        CrosVmParam::List(list) => {
            let mut sub_object = vec![];

            for item in list {
                // Each sub object of a list could be a value, bool, map, or another list
                sub_object.push(match_cfg_key(item, data));
            }

            sub_object.into()
        }
    }
}

pub fn generate_cfg(vm_spec: &dyn vm::CrosVmConfig) -> String{
    let mut data = json::JsonValue::new_object();

    let cfg_data = vm_spec.generate_config();

    for cfg in cfg_data{
        data[cfg.name.as_str()] = match_cfg_key(cfg.params, &mut data);
    }

    json::stringify(data)
}