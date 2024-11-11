use crate::vm::{CrosVmCmdLine, CrosVmConfig, CrosVmParam};
use json;
/*
A struct to represent a crosvm block device
 */
#[derive(Default, Clone, Eq, PartialEq, Debug)]
pub struct Block{
    pub path: String,
    pub root: bool
}

/*
A static definition of a crosVM linux configuration, this is done to main our own stable, defined
interface on top of a stable freeze of crosvm.
*/
#[derive(Default, Clone, Eq, PartialEq, Debug)]
pub struct Vm{
    // Required Parameters
    /*
    Path to the kernel image used to boot the VM
     */
    pub kernel: String,

    // Everything optional
    /*
    Kernel parameters
    (Optional)
     */
    pub kernel_params: Vec<String>,
    /*
    Path to the initrd used to boot the VM
    (Optional)
     */
    pub initrd: Option<String>,
    /*
    Block devices to attach to the VM
    (Optional)
     */
    pub disks: Vec<Block>
}

impl CrosVmConfig for Vm{
    fn generate_config(&self) -> Vec<CrosVmCmdLine> {
        let mut config = vec![];
        config.push(CrosVmCmdLine{
            name: String::from("kernel"),
            params: CrosVmParam::Value(self.kernel.clone()),
        });

        if self.kernel_params.len() > 0 {
            config.push(CrosVmCmdLine {
                name: String::from("params"),
                params: CrosVmParam::List({
                    let mut values = vec![];
                    for param in self.kernel_params.clone(){
                        values.push(param.into());
                    }
                    values
                }),
            });
        }

        if self.initrd.is_some(){
            config.push(CrosVmCmdLine{
                name: String::from("initrd"),
                params: CrosVmParam::Value(self.initrd.as_ref().unwrap().clone()),
            });
        }

        if self.disks.len() > 0 {
            config.push(CrosVmCmdLine {
                name: String::from("block"),
                params: CrosVmParam::List({
                    let mut block_data = vec![];
                    for block in self.disks.clone() {
                        block_data.push(CrosVmParam::Map(
                            vec![
                                (String::from("path"), block.path.into()),
                                (String::from("root"), block.root.into())
                            ]
                        ))
                    }
                    block_data
                }),
            });
        }

        config
    }

    fn generate_vm(&mut self, cfg: String) {
        let mut json_config = json::parse(cfg.as_str())
            .expect("Unable to parse VM config!");

        self.kernel = {
            // The kernel is a required parameter, we should assume it exists
            let kernel = &mut json_config["kernel"];
            assert!(kernel.is_string());
            kernel.take_string()
                .expect("Unable to get string from kernel!")
        };
        self.kernel_params = {
            // The kernel parameters are a list of strings which represent the various params
            let params = &mut json_config["params"];
            if !params.is_array(){
                // If it's anything but an array, assume it's empty
                vec![]
            }else {
                let mut kernel_params = vec![];
                let mut value = params.pop();
                while value.is_string() {
                    kernel_params.push(value.take_string()
                        .expect("Found non-string while looking for kernel parameters!"));
                    value = params.pop();
                }
                kernel_params
            }
        };
        self.initrd = {
            let initrd = &mut json_config["initrd"];
            // If it isn't a string, assume it doesn't exist
            if !initrd.is_string(){
                None
            } else{
                Some(initrd.take_string()
                    .expect("Failed to take initrd string!"))
            }
        };

        self.disks = {
            // The disks are a list of json values that are maps of the various parameters
            let params = &mut json_config["block"];
            if !params.is_array(){
                // If it's anything but an array, assume it's empty
                vec![]
            }else {
                let mut block_devices = vec![];
                let mut value = params.pop();
                while value.is_object() {
                    let path = value["path"].take_string()
                        .expect("Couldn't find path to block device!");
                    let root = value["root"].as_bool()
                        .unwrap_or(false);

                    block_devices.push(Block{
                        path,
                        root
                    });
                    value = params.pop();
                }
                block_devices
            }
        };
    }
}

// Tests for snix
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    #[test]
    fn test_generate_vm(){
        let test_vm = Vm{
            kernel: "./bzImage".to_string(),
            kernel_params: vec!["test=value".to_string()],
            initrd: Some("./initrd.img".to_string()),
            disks: vec![
                Block{
                    path: "./rootfs".to_string(),
                    root: true,
                }
            ],
        };
        let mut other_vm = Vm::default();
        other_vm.generate_vm(generate_cfg(&test_vm));

        assert_eq!(test_vm, other_vm)
    }
}