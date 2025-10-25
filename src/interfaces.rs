use alloy::sol;

sol! {
 #![sol(all_derives, rpc)]


    interface IzQuoter {
        enum AMM {
            UNI_V2,
            AERO,
            ZAMM,
            UNI_V3,
            UNI_V4,
            AERO_CL
        }

        struct Quote {
            AMM source;
            uint256 feeBps;
            uint256 amountIn;
            uint256 amountOut;
        }

        function buildBestSwap(
            address to,
            bool exactOut,
            address tokenIn,
            address tokenOut,
            uint256 swapAmount,
            uint256 slippageBps,
            uint256 deadline
        ) external view returns (Quote memory best, bytes memory callData, uint256 amountLimit, uint256 msgValue);

        function buildBestSwapViaETHMulticall(
            address to,
            address refundTo,
            bool exactOut,
            address tokenIn,
            address tokenOut,
            uint256 swapAmount,
            uint256 slippageBps,
            uint256 deadline
        )
            external
            view
            returns (Quote memory a, Quote memory b, bytes[] memory calls, bytes memory multicall, uint256 msgValue);
    }


    interface ISwapSwap {
        error SwapSwap__ZeroBalance();
        error SwapSwap__ZeroAddress();
        error SwapSwap__ETHTransferFailed();
        error SwapSwap__SwapFailed();
        error SwapSwap__SetApprovalFailed(address token, uint256 amount);

        event SwapSwap__zRouterUpdated(address indexed _zRouter);
        event SwapSwap__TokenRecovered(address indexed token);
        event SwapSwap__ETHRecovered();
        event SwapSwap__SwapExecuted(address indexed tokenIn, uint256 indexed amountIn, uint256 indexed amountOut);
        event SwapSwap__CallDataSwapExecuted(bytes indexed data, bytes indexed returnedData);

        function setApproval(address token, uint256 amount) external;

        function setRouter(address _zRouter) external;

        function executeSwap(bytes calldata data) external;

        function executeCLSwap(bytes calldata data) external;

        function executeCallDataSwap(bytes calldata data, uint256 msgValue) external;

        function recoverToken(address token) external;

        function recoverETH() external;

        function i_token() public view returns(address);
    }


}
