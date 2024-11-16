# vSnix-VMM Architecture

# IN PROGRESS, VERY INCOMPLETE

## Server Implementation
The following describes how the server will be implemented in the vSnix-VMM binary. This differs from the spec of the
server itself as that does not describe how the server should handle vm pools, listening on sockets, threads, etc...

### cvm_server
The server must be able to create and handle socket connections, this also infers that the server must be able the
inferred features of the server such as vm pools.

The primary server struct will manage configuration files to load VM pools across launches of the application along with
managing sockets persistent across reboots.

#### VM Pools: From the file system perspective
Each VM pool should be a raw image file with an ext4 file system. Along with a configuration file for the associated VM
pool.

From the file system, this should appear as:
* [POOL_NAME].img
* [POOL_NAME].json

Where [POOL_NAME] refers to an ASCII representation of the pool name.

When [POOL_NAME].img is mounted, the VM pool (i.e. root of the file system) should have the following file structure:

* vms.json
* [HASH]_vmcfg.cfg
* kernels/
    * kernels/[HASH].vmLinuz
    * kernels/[HASH].initrd.img
* blocks/
    * blocks/[HASH].root.img

Where [HASH] refers to a hash of the Virtual Machine's name (The hash can be built any way the server desires, here it 
is built using Rust's built-in hashing system)

#### VM Pools: From the program's perspective
A VM pool should be an instance of the vm_pool struct which should have an impl block. To initialize a VM pool, the
following flow of actions should happen within the program:
1. An instance of the VM pool is created with a provided path to the location of VM pools along with the name of the
desired VM Pool
    * The instance should check for the existence of the pool image and configuration, if at least one doesn't exist
the code should terminate as it is out of the scope of the VM pool loader to create VM pools.
2. The newly loaded instance should be provided a mount point for the VM pool and proceed to attempt to execute the
mount command to mount the vm image to the desired mount point.
    * The VM Pool image should be ext4 but the VM pool doesn't actually care as long mount executes without error.
3. The VM pool loader should read the vms.json file located in the root of the VM Pool.
4. Using the information from vms.json, the VM pool should load each VM config into memory

At this point, the VM pool loader should be ready to deliver configuration information for virtual machines to the
server.