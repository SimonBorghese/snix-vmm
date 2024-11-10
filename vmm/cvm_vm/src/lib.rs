use crate::vm::{CrosVmCmdLine, CrosVmParam};

pub mod vm;
pub mod snix;

fn match_cfg_key(cmd_line: CrosVmParam, data: &mut json::JsonValue) -> json::JsonValue{
    match cmd_line{
        CrosVmParam::Value(Value) => {
            Value.into()
        }
        CrosVmParam::Bool(Value) => {
            Value.into()
        }
        CrosVmParam::Map(Map) => {
            let mut sub_object = json::JsonValue::new_object();
            for item in Map{
                sub_object[item.0] = match_cfg_key(item.1, data);
            }
            sub_object
        }
        CrosVmParam::List(List) => {
            let mut sub_object = vec![];

            for item in List{
                sub_object.push(match_cfg_key(item, data));
            }

            sub_object.into()
        }
    }
}

pub fn generate_cfg(vm_spec: &dyn vm::CrosVmConfig){
    let mut data = json::JsonValue::new_object();

    let cfg_data = vm_spec.generate_config();

    for cfg in cfg_data{
        data[cfg.name.as_str()] = match_cfg_key(cfg.params, &mut data);
    }

    println!("{}", json::stringify(data));
}