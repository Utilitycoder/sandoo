pragma solidity ^0.8.0;

import {Test} from "forge-std/Test.sol";
import {console} from "forge-std/console.sol";

import "../src/Sandooo.sol";

interface IERC20 {
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

    function name() external view returns (string memory);

    function symbol() external view returns (string memory);

    function decimals() external view returns (uint8);

    function totalSupply() external view returns (uint256);

    function balanceOf(address account) external view returns (uint256);

    function transfer(address to, uint256 value) external returns (bool);

    function allowance(address owner, address spender) external view returns (uint256);

    function approve(address spender, uint256 value) external returns (bool);

    function transferFrom(address from, address to, uint256 value) external returns (bool);
}

interface IWETH is IERC20 {
    function deposit() external payable;

    function withdraw(uint256 amount) external;
}

interface IUniswapV2Pair {
    function token0() external returns (address);

    function token1() external returns (address);

    function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast);
}

// anvil --fork-url http://localhost:8545 --port 2000
// forge test --fork-url http://localhost:2000 --match-contract SandoooTest -vv
contract SandoooTest is Test {
    Sandooo bot;
    IWETH weth = IWETH(0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2);

    receive() external payable {}

    function test() public {
        console.log("Sandooo bot test starting");

        // Create Sandoo instance
        bot = new Sandooo();

        uint256 amountIn = 100000000000000000; // 0.1 ETH

        // Wrap ETH to WETH and send to the Sandooo contract
        weth.deposit{value: amountIn}();
        weth.transfer(address(bot), amountIn);

        // Check if WETH is properly sent
        uint256 botBalance = weth.balanceOf(address(bot));
        console.log("Bot WETH balance: %s", botBalance);

        // Check if we can recover ETH
        bot.recoverToken(address(weth), botBalance);
        uint256 botBalanceAfterRecover = weth.balanceOf(address(bot));
        console.log("Bot WETH balance after recover: %s", botBalanceAfterRecover);

        // Check if we can recover ETH
        (bool success,) = address(bot).call{value: amountIn}("");
        console.log("ETH recover success: %s", success);
        uint256 testEthBal = address(this).balance;
        uint256 botEthBal = address(bot).balance;
        console.log("Test ETH balance: %s", testEthBal);
        console.log("Bot ETH balance: %s", botEthBal);

        console.log("============================");

        // Send WETH to the contract again
        weth.transfer(address(bot), amountIn);
        uint256 startingWethBalance = weth.balanceOf(address(bot));
        console.log("Starting WETH balance: %s", startingWethBalance);

        address usdt = 0xdAC17F958D2ee523a2206206994597C13D831ec7;
        address wethUsdtV2 = 0x0d4a11d5EEaaC28EC3F61d100daF4d40471f1852;

        IUniswapV2Pair pair = IUniswapV2Pair(wethUsdtV2);
        address token0 = pair.token0();
        address token1 = pair.token1();

        // We will be testing WETH to USDT
        // So it's zeroForOne is WETH is token0
        uint8 zeroForOne = token0 == address(weth) ? 1 : 0;

        // calculate the amountOut using reserves
        (uint112 reserve0, uint112 reserve1,) = pair.getReserves();

        uint256 reserveIn;
        uint256 reserveOut;

        if (zeroForOne == 1) {
            reserveIn = reserve0;
            reserveOut = reserve1;
        } else {
            reserveIn = reserve1;
            reserveOut = reserve0;
        }

        uint256 amountInWithFee = amountIn * 997;
        uint256 numerator = amountInWithFee * reserveOut;
        uint256 denominator = reserveIn * 1000 + amountInWithFee;
        uint256 targetAmountOut =numerator / denominator;

        console.log("Amount in: %s", amountIn);
        console.log("Target Amount out: %s", targetAmountOut);

        bytes memory data = abi.encodePacked(
            uint64(block.number),
            uint8(zeroForOne),
            address(pair),
            address(weth),
            uint256(amountIn),
            uint256(targetAmountOut)
        );

        console.log("Calldata");
        console.logBytes(data);

        uint gasBefore = gasleft();
        (bool s, ) = address(bot).call(data);
        uint gasAfter = gasleft();
        uint gasUsed = gasBefore - gasAfter;
        console.log("Swap Success:  %s", s);
        console.log("Gas used: %s", gasUsed);

        uint256 usdtBalance = IERC20(usdt).balanceOf(address(bot));
        console.log("USDT bot balance: %s", usdtBalance);

        require(s, "Swap failed");
    }
}
