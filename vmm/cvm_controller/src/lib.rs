use vm_control as cvm;
use vm_control::VmRequest;

pub struct VmSocket{
    socket: Option<std::path::PathBuf>
}

impl VmSocket{
    // Private Utility functions
    fn get_socket_path(&self) -> &std::path::PathBuf{
        self.socket.as_ref().expect("No socket loaded into VmSocket!")
    }

    fn command_vm(&self, vm_request: VmRequest, error: String) -> Result<(), String>{
        let socket_string = self.get_socket_path();

        let result = cvm::client::vms_request(&vm_request, socket_string);

        if result.is_ok(){
            Ok(())
        } else{
            Err(error)
        }
    }

    // Public API
    pub fn new() -> Self{
        Self{socket:None}
    }
    pub fn connect(mut self, path: &std::path::Path) -> Result<Self, String>{
        // Check if the socket path actually points to something
        let test_socket_path = std::path::PathBuf::from(path);
        let socket_exists = std::fs::exists(test_socket_path.clone())
            .expect("Unable to try and find socket");

        if !socket_exists{
            return Err(format!("Socket {} doesn't exist!", test_socket_path.to_string_lossy()))
        }

        self.socket = Some(test_socket_path);

        // Currently, this doesn't guarantee the socket will actually work, this just verifies it
        // exists which should hopefully catch 99% of misconfigurations
        Ok(self)
    }

    /*
    Suspend the VM's vCPUs
    This method is preferred over fully suspending the VM due to lack of implementation
    for the sleep function in some devices (usually resulting in a crash when suspend_vm_full
    is used).
     */
    pub fn suspend_vm(&self) -> Result<(), String>{
        self.command_vm(VmRequest::SuspendVcpus, String::from("Failed to suspend VM vCPUs!"))
    }

    /*
    Fully suspend all VM devices
    This is significantly less stable than suspend_vm as not all devices (including default ones
    like xhci) don't fully implement the Suspendable interface and therefore cause a crash
     */
    pub fn suspend_vm_full(&self) -> Result<(), String>{
        self.command_vm(VmRequest::SuspendVm, String::from("Failed to fully suspend VM!"))
    }

    /*
    Resume the VM's vCPUs
    As stated with suspend_vm, using this path is significantly more reliable than trying to fully
    suspend the VM
     */
    pub fn resume_vm(&self) -> Result<(), String>{
        self.command_vm(VmRequest::ResumeVcpus, String::from("Failed to resume VM vCPUs!"))
    }

    /*
    Fully resume the VM
    Presuming the VM could successfully be suspended without a crash (which I have yet to
    accomplish) it would be presumed that this function should not fail.
     */
    pub fn resume_full_vm(&self) -> Result<(), String>{
        self.command_vm(VmRequest::ResumeVm, String::from("Failed to fully resume VM!"))
    }

    /*
    Stop the VM
    Self-explanatory
     */
    pub fn stop_vm(&self) -> Result<(), String>{
        self.command_vm(VmRequest::Exit, String::from("Failed to exit VM!"))
    }
}
