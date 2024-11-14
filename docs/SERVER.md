# vSnix-VMM Server Architecture
The vSnix-VMM works on a server-client relationship between the base OS and the guest operating systems. Communication
is done with Unix sockets where a server can have multiple sockets with differing levels of permissions.

## BaseOS Server
Manages the functions of the primary server, including managing running virtual machines and lists of installed virtual
machines. This does not have an extern socket however for security purposes, instead that is relegated to the UserOS
sockets.

## UserOS Sockets
Instead of exposing the entire servet to UserOS clients of the hypervisor, instead each UserOS virtual machine who has
been configured to be able to use the hypervisor will receive their own socket, list of virtual machines, and
permissions for how to interface with their own virtual machines.

### UserOS Socket API
#### Socket API structure
The socket API is structured by the following specification

Parameters to the socket are a comma seperated list of values, seperated by new lines, all commands sent to the socket 
will always begin with the format:

`[COMMAND],[COMMAND VERSION]`

`[COMMAND]` is an integer representing the Socket API command. Command IDs, once defined in a stable release, will 
never be changed, only deprecated but a command will never replace an existing command's ID.

`[COMMAND VERSION]` is an integer representing the command's version. The version of a command defines, parameters,
how parameters are interpreted, and the outputs.

Parameters following the command are seperated by new lines such that each line contains either, the command itself or
a parameter defined by the previously defined command.

Only one command should be sent at once and the next command should only be sent once the output from the previous
command is received.

##### Example Command
```
1,3           # Command #1, version 3
10,Cool Param # String size, String
1234          # Integer parameter
20.0          # Floating point parameter
```

#### Permissions
The following defines the various permissions that can be assigned to a socket.
Each permission is associated with a byte from a 16-bit unsigned integer such that bitwise operations on a single
integer can determine the permissions of a socket
##### Example
* `[Permission Name] [Bitwise operation to determine bit]`
    * `[PERMISSION DESCRIPTION]`

##### Definitions
* PERM_VIEW_LOCAL_STATE `[1 << 0]` 
  * View the local socket's permissions and assigned limits
* PERM_MANAGE_LOCAL_VM `[1 << 1]`
  * View, Create, Modify, and Delete virtual machine definitions on the hypervisor
  owned by this socket
* PERM_RUN_LOCAL_VM `[1 << 2]`
  * Manage the execution state virtual machine definitions on the hypervisor owned by the socket.
* PERM_MANAGE_ALL_VM `[1 << 3]`
  * View, Create, Modify, and Delete ALL Virtual Machines defined on all sockets (Dangerous)
* PERM_RUN_ALL_VM `[1 << 4]`
  * Manage the execution state of ALL virtual machines defined on all sockets (Dangerous) 

#### Socket API
The following defines the API for the sockets

---

##### Example API, Version 3
* Command Number: 0
* Required Permissions: PERM_EXAMPLE
* Inputs
  * String saying XYZ up to max length u16_MAX
  * Integer Parameter meaning ABC
  * Floating Point Parameter meaning EFG
* Outputs
  * Integer Returning Success of operation

* Example Input
```
0,3           # Command #1, version 3
10,Cool Param # String size, String
1234          # Integer parameter
20.0          # Floating point parameter
```
* Example Output
```
0,3
0             # Success
```

---

##### Get Socket Permissions, Version 1
* Command Number: 1
* Required Permissions: PERM_VIEW_LOCAL_STATE
* Inputs
    * None
* Outputs
    * u16 integer containing the socket's permissions

* Example Input
```
1,1
```
* Example Output
```
1,1
7   # 00...111 (PERM_VIEW_LOCAL_STATE, PERM_MANAGE_LOCAL_VM, PERM_RUN_LOCAL_VM)
```

---

##### Get Local VM Count, Version 1
* Command Number: 2
* Required Permissions: PERM_MANAGE_LOCAL_VM
* Inputs
    * None
* Outputs
    * u16 integer reporting the number of registered local virtual machines

* Example Input
```
2,1
```
* Example Output
```
2,1
4   # 4 Local Virtual Machines
```

---

##### Get Local UID From Index, Version 1
* Command Number: 3
* Required Permissions: PERM_MANAGE_LOCAL_VM
* Inputs
    * String containing a semicolon seperated list of the desired VM indices to receive UIDs for
* Outputs
    * String containing a semicolon seperated list of UIDs OR an error string WITHOUT any semicolons

* Example Input
```
3,1
9,1;2;3;4;7     # Lookup the UIDs for VMs at indices 1,2,3,4,7
```
* Example Output
```
3,1
24,ABCD1;ABCD2;ABCD3;ABCD4;ABCD7
```
OR
```
3,1
16,NO VM AT INDEX 7     # Note the lack of semicolons in the output
```

---

##### Get Local VM UIDs, Version 1
* Command Number: 4
* Required Permissions: PERM_MANAGE_LOCAL_VM
* Inputs
    * None
* Outputs
    * String containing a semicolon seperated list of UIDs OR an error string WITHOUT any semicolons

* Example Input
```
4,1
```
* Example Output
```
4,1
24,ABCD1;ABCD2;ABCD3;ABCD4;ABCD7
```
OR
```
4,1
12,NO VMS FOUND     # Note the lack of semicolons in the output
```

---

##### Get Local VM Config, Version 1
* Command Number: 5
* Required Permissions: PERM_MANAGE_LOCAL_VM
* Inputs
    * String containing a semicolon seperated list of UIDs to receive configs for
* Outputs
    * String containing a semicolon seperated list of Configs OR an error string WITHOUT any semicolons

* Example Input
```
5,1
24,ABCD1;ABCD2;ABCD3;ABCD4;ABCD7
```
* Example Output
```
5,1
9999,[CONFIG ABCD1];[CONFIG ABCD2];[CONFIG ABCD3];[CONFIG ABCD4];[CONFIG ABCD7]
```
OR
```
5,1
12,NO UID ABCD7    # Note the lack of semicolons in the output
```

---

##### Create Local VM, Version 1
* Command Number: 6
* Required Permissions: PERM_MANAGE_LOCAL_VM
* Inputs
    * String containing a human-readable name for the VM
* Outputs
    * String containing a fresh UID for the new VM OR an error string WHERE the error string's length is NOT a multiple
of 2

* Example Input
```
6,1
6,CoolVM
```
* Example Output
```
6,1
16,ABCDEFGHIJKLMNOP
```
OR
```
6,1
13,INVALID NAME!    # Note the length of the string is NOT divisible by 2
```

---

##### Get Local VM Max Kernel Size, Version 1
* Command Number: 7
* Required Permissions: PERM_VIEW_LOCAL_STATE
* Inputs
    * None
* Outputs
    * An integer up to U32_MAX reporting the max uploadable kernel size for this VM in bytes

* Example Input
```
7,1
```
* Example Output
```
7,1
64000000    # 64 megabytes in bytes
```

---

##### Upload Local VM Kernel, Version 1
* Command Number: 8
* Required Permissions: PERM_VIEW_LOCAL_STATE, PERM_MANAGE_LOCAL_VM
  * PERM_VIEW_LOCAL_STATE is required as an attacker can interpolate the max kernel size from brute force from errors
* Inputs
    * String containing the target VM's UID
    * A Buffer containing the desired kernel up until the maximum kernel size
* Outputs
    * A String either empty or reporting any errors

* Example Input
```
8,1
5,ABCD1
9999,[KERNEL BUFFER]
```
* Example Output
```
8,1
0,  # Note the lack of string meaning no error
```
OR
```
8,1
14,KERNEL TOO BIG
```

---

##### Execute VM, Version 1
* Command Number: 9
* Required Permissions: PERM_MANAGE_LOCAL_VM, PERM_RUN_LOCAL_VM
* Inputs
    * String containing the target VM's UID
    * String containing the kernel parameters
* Outputs
    * String pointing to a virtual, attached virtual serial console, OR an error where the error will lack the '/'
character

* Example Input
```
9,1
5,ABCD1
14,root=/dev/vda5
```
* Example Output
```
9,1
17,/tmp/ABCD1-vttyS0
```
OR
```
9,1
15,VM DOESNT EXIST  # Note the lack of the '/' character in the output as if it is not a file path
```