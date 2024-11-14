/*
An implementation of the UNIX socket interface for the server
This not only provides the API for creating a socket but also useful information for clients for
implementing interfaces with the socket
 */

/*
See docs/SERVER.md for details
 */

pub enum SocketPermissions{
    ViewLocalState = 1 << 0,
    ManageLocalVm = 1 << 1,
    RunLocalVm = 1 << 2,
    ManageAllVm = 1 << 3,
    RunAllVm = 1 << 4,
}

pub enum SocketCommandIds{
    Reserved = 0,

    GetSocketPermissions = 1,

    GetLocalVmCount = 2,

    GetLocalUIDFromIndex = 3,

    GetLocalVmUIDs = 4,

    GetLocalVmConfig = 5,

    CreateLocalVm = 6,

    GetLocalVmMaxKernelSize = 7,

    UploadLocalVmKernel = 8,

    ExecuteVm = 9
}