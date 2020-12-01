# Distaff VM
Distaff is a zero-knowledge virtual machine written in Rust. For any program executed on Distaff VM, a STARK-based proof of execution is automatically generated. This proof can then be used by anyone to verify that a program was executed correctly without the need for re-executing the program or even knowing what the program was.

The original Distaff VM repo can be found [here](https://github.com/GuildOfWeavers/distaff). 

# The STARK proof verification module
We only keep the STARK proof verification function and the related packages in this module. The STARK proof generation function in the original Distaff VM has been removed—as proof generation takes place at the client side, not on chain.

### Status
**DO NOT USE IN PRODUCTION.** The work is in an alpha stage. This means that current functionality is incomplete, and there are known and unknown bugs and security flaws.
