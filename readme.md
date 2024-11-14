# The vSnix Project
Welcome to the GitHub page for the vSnix VMM project.

# This project is not ready for use, what you see here is provisional

## What is the vSnix Project
The vSnix project (Virtualized, Secure *nix) is a virtualization-based solution for running secure operating systems.

### vSnix Project Components
* vSnix: A type-1 hypervisor built upon an immutable Linux image w/ KVM (based on buildroot)
* **vSnix-VMM**: A virtual machine manager fo vSnix designed to provide a programmable interface between vSnix and the guest
* vSnix-OS: A configuration for vSnix built upon the principles of [Qubes-OS](https://www.qubes-os.org/) where PCIe
components are isolated in virtual machines, designed to provide a more secure platform for operating systems.

## How to use
The vSnix-VMM is built around a single binary for both its server, and CLI/GUI clients. To test this binary, first sync
the git submodules, then navigate to the `vmm` folder and run `cargo build`, the resultant binary will be located at 
`target/debug/snix-vmm`