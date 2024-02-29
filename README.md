# serverlessinterface
Serverless interface 

This is a repo build for running the serverless experiments. It act as a middleinterface and configuration system for firecracker VMs.

## Current work progress:
Iplibrary

```Rust
vmconfig::set_up_vm(IpLibrary);
 ->firecrackerapi::initilize_vm(VMsetup,IpLibrary);
    ->set_boot_source()
      ->set_rootfs()
        ->set_network()
          ->register_network()

```
## Designed Features:

High parallization

Fuse support

Encryption for memory blocks

REAP functionality

Memory loading scheme


## Underwork parts:

A lot :(

## Datastructrures:
