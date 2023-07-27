// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.2 <0.9.0;

contract Erc20 {

   string public name;
   string public symbol;
   uint8 public decimals;
   address public admin;
   address public balances;
   uint256 public total_supply;
   string public allowances;

   constructor(string memory name, string memory symbol, uint8 decimals, address admin) {}

   function mint(address beneficiary, uint256 amount) public {}

   function transfer_from(address sender, address beneficiary, uint256 amount) public {}

   function transfer(address to, uint256 amount) public {}

   function balance_of(address account) public {
   }

   function decimals() public {}

   function allowance(address owner, address spender) public {}

   function approve(address spender, uint256 amount) public {}
}
