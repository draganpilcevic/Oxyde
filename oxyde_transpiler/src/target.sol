// SPDX-License-Identifier: GPL-3.0

pragma solidity >=0.8.2 <0.9.0;

/**
 * @title Storage
 * @dev Store & retrieve value in a variable
 * @custom:dev-run-script ./scripts/deploy_with_ethers.ts
 */
contract SampleContract {

    uint64 public field_a;
    uint64 field_b;

    constructor(uint64 val_a, uint64 val_b) {
        field_a = val_a;
        field_b = val_b;
    }

    function set_a(uint64 val) public {
        field_a = val;
    }
}