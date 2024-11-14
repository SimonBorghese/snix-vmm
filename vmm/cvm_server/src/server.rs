/*
CvmServer
The primary server struct for the appication, this is not used directly by the application besides
during initialization but rather a CvmServerSocket provides the actual APIs for use with the
application. This is done to manage permissions.
 */
use std::io::Read;
use cvm_vm;
use cvm_vm::vm::CrosVmConfig;

#[derive(Default, Clone, Debug)]
pub struct CvmServer<'a>{
    snix_vms: Vec<&'a cvm_vm::snix::Vm>
}

impl<'a> CvmServer{
    // Private Utility functions
    fn load_vm_from_file(cfg_file: &std::path::Path) -> Result<cvm_vm::snix::Vm, String>{
        if !cfg_file.exists() || !cfg_file.is_file() {
            return Err(format!("{:?} is not a file.", &cfg_file));
        }

        let mut file = std::fs::File::open(cfg_file)
            .expect("Unable to open file!");

        let mut file_buffer: String = String::default();
        file.read_to_string(&mut file_buffer)
            .expect("Unable to read file to string!");

        let mut vm = cvm_vm::snix::Vm::default();
        vm.generate_vm(file_buffer);

        Ok(vm)
    }

}