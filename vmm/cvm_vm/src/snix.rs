use crate::vm;
use crate::vm::{CrosVmCmdLine, CrosVmConfig, CrosVmParam};
/*
A struct to represent a crosvm block device
 */
#[derive(Default, Clone)]
pub struct Block{
    pub path: String,
    pub root: bool
}

/*
A static definition of a crosVM linux configuration, this is done to main our own stable, defined
interface on top of a stable freeze of crosvm.
*/
#[derive(Default, Clone)]
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
}