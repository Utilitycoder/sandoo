// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

interface IERC20 {
    function name() external view returns (string memory);
    function symbol() external view returns (string memory);
    function decimals() external view returns (uint8);
    function totalSupply() external view returns (uint256);
}

contract Request {
    function getTokenInfo(address targetToken)
        external
        view
        returns (string memory name, string memory symbol, uint8 decimals, uint256 totalSupply)
    {
        IERC20 token = IERC20(targetToken);

        name = token.name();
        symbol = token.symbol();
        decimals = token.decimals();
        totalSupply = token.totalSupply();
    }
}
