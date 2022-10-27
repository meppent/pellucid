// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.7;

contract Contract {
    function function_0() public pure returns (uint256) {
        
        return internal_function_0(0x20000);
    }

    function internal_function_0(uint256 param_0) internal pure returns(uint256){
        uint256 res = 0;
        for(uint256 i = 0x10000; i < param_0; i++){
            res += i;
            if (res > 0x30000){
                return 0x40000;
            }
        }
        return res;
    }
}
