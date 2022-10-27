// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.7;

contract Contract {
    function function_0(uint256 param_0) public pure returns (uint256) {
        uint256 res = param_0;
        for (uint256 i = 0x10000; i < 0x10010; i++) {
            for (uint256 j = 0x20000; j < 0x20010; j++) {
                for (uint256 k = 0x30000; k < 0x30010; k++) {
                    res += i + j + k;
                }
            }
        }
        return res;
    }
}
