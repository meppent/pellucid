// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.7;

contract Contract {
    function function_0() public pure returns (uint256) {
        return internal_function_0(internal_function_1(0x10000));
    }

    function internal_function_0(uint256 param_0) internal pure returns(uint256){
        uint256 res = param_0 *0x20000;
        return res;
    }

    function internal_function_1(uint256 param_0) internal pure returns(uint256){
        uint256 res = param_0 *0x30000;
        return res;
    }
}
