// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract SimpleContract {
    uint256 public stor;

    function f() external returns (uint256) {
        uint256 length = 12;
        for (uint256 i = 0; i < length; i++) {
            bool res = intern(i);
            stor = res ? i : i;
        }
        return 55 + 12;
    }

    function g() external returns (bool) {
        stor = 66666666666;
        uint256 i = 0;
        while (!intern(i)) {
            unchecked {
                ++i;
            }
        }
        return intern(i);
    }

    function intern(uint256 arg) internal pure returns (bool) {
        return arg > 6;
    }
}
