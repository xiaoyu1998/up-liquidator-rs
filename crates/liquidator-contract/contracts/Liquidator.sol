// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.24;

import "@openzeppelin/contracts/access/Ownable.sol";
import '@openzeppelin/contracts/token/ERC20/IERC20.sol';
import '@uniswap/v3-core/contracts/interfaces/callback/IUniswapV3SwapCallback.sol';
import '@uniswap/v3-core/contracts/interfaces/IUniswapV3Pool.sol';
import './Interface.sol';
import './PoolAddress.sol';
import "./Printer.sol";

// Uncomment this line to use console.log
// import "hardhat/console.sol";

contract Liquidator is Ownable, IUniswapV3SwapCallback {

    /// @dev The minimum value that can be returned from #getSqrtRatioAtTick. Equivalent to getSqrtRatioAtTick(MIN_TICK)
    uint160 internal constant MIN_SQRT_RATIO = 4295128739;
    /// @dev The maximum value that can be returned from #getSqrtRatioAtTick. Equivalent to getSqrtRatioAtTick(MAX_TICK)
    uint160 internal constant MAX_SQRT_RATIO = 1461446703485210103287273052203988822378723970342;

    address internal immutable factoryUniswapV3;
    address internal immutable exchangeRouter;
    address internal immutable reader;
    address internal immutable dataStore;
    address internal immutable liquidationHandler;

    constructor(
        address _factory, 
        address _dataStore,
        address _exchangeRouter, 
        address _reader,
        address _liquidationHandler
    ) Ownable(msg.sender) { 
        factoryUniswapV3 = _factory;
        dataStore = _dataStore;
        exchangeRouter = _exchangeRouter;
        reader = _reader;
        liquidationHandler = _liquidationHandler;
    }

    function getPool(
        address tokenA,
        address tokenB,
        uint24 fee
    ) private view returns (IUniswapV3Pool) {
        return IUniswapV3Pool(PoolAddress.computeAddress(factoryUniswapV3, PoolAddress.getPoolKey(tokenA, tokenB, fee)));
    }

    struct LiquidateVars {
        uint256 usdBalanceBeforeLiquidation;
        uint256 usdBalanceAfterLiquidation;
        uint256 balanceAfterSwap;
        GetLiquidationHealthFactor factor;
        uint256 i;
        Asset debt;
        Asset collateral;
        IUniswapV3Pool pool;
        bool zeroForOne;
        uint256 debtAmount;
        ExecutionLiquidationParams executionLiquidationParams;
        int256 usdGain;
    } 

    function liquidate(
        LiquidationParams calldata params
    // ) external onlyOwner returns (int256) {
    ) external onlyOwner {
        LiquidateVars memory vars;
        vars.usdBalanceBeforeLiquidation = IERC20(params.usdToken).balanceOf(address(this));
        Printer.log("unclaimedFee", vars.usdBalanceBeforeLiquidation);
        vars.factor = IReader(reader).getLiquidationHealthFactor(dataStore, params.account);
        require(!vars.factor.isHealthFactorHigherThanLiquidationThreshold, 
            "Liquidator: The health factor is higher than the liquidation threshold");

        //buy debts
        for (vars.i = 0; vars.i < params.debts.length; vars.i ++) {
            vars.debt = params.debts[vars.i];
            vars.debtAmount = IReader(reader).getDebt(dataStore, params.account, vars.debt.token);
            IERC20(vars.debt.token).approve(liquidationHandler, vars.debtAmount);
            if( params.usdToken == vars.debt.token ){ continue; }

            vars.pool = getPool(params.usdToken, vars.debt.token, params.uniswapFee);
            vars.zeroForOne = params.usdToken < vars.debt.token;           
            vars.pool.swap(
                address(this),
                vars.zeroForOne,
                -int256(vars.debtAmount),
                vars.zeroForOne ? MIN_SQRT_RATIO + 1 : MAX_SQRT_RATIO - 1,
                abi.encode(params.usdToken, vars.debt.token, params.uniswapFee)
            );

            vars.balanceAfterSwap = IERC20(vars.debt.token).balanceOf(address(this));
            require(vars.balanceAfterSwap == vars.debtAmount, 
                "Liquidator: Do not swap to get enough asset ");           

        }

        vars.executionLiquidationParams = ExecutionLiquidationParams(
            params.account
        );

        IExchangeRouter(exchangeRouter).executeLiquidation(vars.executionLiquidationParams);

        //sell collaterals
        for (vars.i = 0; vars.i < params.collaterals.length; vars.i ++) {
            vars.collateral = params.collaterals[vars.i];
            if( params.usdToken == vars.collateral.token ){ continue; }

            vars.pool = getPool(params.usdToken, vars.collateral.token, params.uniswapFee);
            vars.zeroForOne = vars.collateral.token < params.usdToken;
            vars.pool.swap(
                address(this),
                vars.zeroForOne,
                int256(vars.collateral.amount),
                vars.zeroForOne ? MIN_SQRT_RATIO + 1 : MAX_SQRT_RATIO - 1,
                abi.encode(params.usdToken, vars.collateral.token, params.uniswapFee)
            );
        }

        vars.usdBalanceAfterLiquidation = IERC20(params.usdToken).balanceOf(address(this));
        vars.usdGain = int256(vars.usdBalanceAfterLiquidation) - int256(vars.usdBalanceBeforeLiquidation) - int256(params.gasFeeUsd) ;


        require(vars.usdGain > 0, "Liquidator: there is no profit of this liquidation action");
        //return vars.usdGain;
    }

    function uniswapV3SwapCallback(
        int256 amount0Delta,
        int256 amount1Delta,
        bytes calldata data
    ) external override {
        (address usdToken, address token, uint24 uniswapFee) = abi.decode(data, (address, address, uint24));
        verifyCallback(factoryUniswapV3, PoolAddress.getPoolKey(usdToken, token, uniswapFee));

        if (amount0Delta > 0) {
            IERC20(IUniswapV3Pool(msg.sender).token0()).transfer(
                msg.sender, 
                uint256(amount0Delta)
            );
        } else if (amount1Delta > 0) {
            IERC20(IUniswapV3Pool(msg.sender).token1()).transfer(
                msg.sender, 
                uint256(amount1Delta)
            );
        } else {
            // if both are not gt 0, both must be 0.
            assert(amount0Delta == 0 && amount1Delta == 0);
        }
    }

    function verifyCallback(address factory, PoolAddress.PoolKey memory poolKey) internal view {
        address p = PoolAddress.computeAddress(factory, poolKey);
        require(msg.sender == p, "invalid pool");
    }

}
