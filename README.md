# Oxyde  

## Overview  

Developing a framework to create smart contracts in Rust for EVM.  

Targets are:  
- Writing smart contracts in Rust  
- Provide an environment to test with cargo (inspired by cw-multi-test), including reentrancy  
- Transpile to Solidity  

Very early work:
- Testing works currently but is not thread-safe (plus writes to disk) 
- Transpiler only generates function signatures  
- No docs or comments  
- oxyde_macros are currently self-contained, but need a clean-up and will use oxyde_parser 
- not all state variables are supported (like tx or msg data)  

## Libs  

### oxyde_macros  
Contains macros definition to generate execute and query entrypoints for a contract plus accessors for public contract fields 

### oxyde_parser  
Utility methods to parse the smart contracts in Rust, and use the parsed data in macros from oxyde_macros and to transpile to Solidity  

### oxyde_sdk  
Definition of types, traits and static elements used for contract development and testing  

### oxyde_transpiler 
Transpiler from Rust to solidity  

### Others  
sample_contracts and oxyde_test are for development purpose and are likely to be removed  
sample_contracts contains a sample ERC20 implementation  

