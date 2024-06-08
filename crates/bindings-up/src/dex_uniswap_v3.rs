pub use dex_uniswap_v3::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod dex_uniswap_v3 {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("tokenA"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("tokenB"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("fee"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint24"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("pool"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getFeeAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getFeeAmount"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint24"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPool"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSqrtPriceLimitX96"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getSqrtPriceLimitX96",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        160usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint160"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSwapFee"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getSwapFee"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("swapExactIn"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("swapExactIn"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sqrtPriceLimitX96"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("swapExactOut"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("swapExactOut"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sqrtPriceLimitX96"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("uniswapV3SwapCallback"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "uniswapV3SwapCallback",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount0Delta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount1Delta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("SwapCallback"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SwapCallback"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount0Delta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount1Delta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("TokenCanNotSwapWithSelf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TokenCanNotSwapWithSelf",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TokenNotMatch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("TokenNotMatch"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pool"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token0"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token1"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static DEXUNISWAPV3_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x0E\xEB8\x03\x80a\x0E\xEB\x839\x81\x01`@\x81\x90Ra\0/\x91a\x01\x0EV[\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x03a\0pW`@Qc;>\xFA\xF9`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x01`@Q\x80\x91\x03\x90\xFD[\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x10a\0\x90W\x82\x84a\0\x93V[\x83\x83[`\x01\x80T`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x96\x87\x16\x17\x90\x91U\x92\x84\x16`\x01`\x01`\xB8\x1B\x03\x19\x90\x91\x16\x17`\x01`\xA0\x1Bb\xFF\xFF\xFF\x96\x90\x96\x16\x95\x90\x95\x02\x94\x90\x94\x17\x90\x93U`\x02\x80T\x90\x93\x16\x91\x16\x17\x90UPa\x01l\x90PV[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\tW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x01$W`\0\x80\xFD[a\x01-\x85a\0\xF2V[\x93Pa\x01;` \x86\x01a\0\xF2V[\x92P`@\x85\x01Qb\xFF\xFF\xFF\x81\x16\x81\x14a\x01SW`\0\x80\xFD[\x91Pa\x01a``\x86\x01a\0\xF2V[\x90P\x92\x95\x91\x94P\x92PV[a\rp\x80a\x01{`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c\xBF\\\x98\"\x11a\0[W\x80c\xBF\\\x98\"\x14a\0\xE2W\x80c\xC1\x84\xA2u\x14a\x01\x05W\x80c\xD6-\x9E\xA3\x14a\x01\x18W\x80c\xFAF\x1E3\x14a\x01+W`\0\x80\xFD[\x80c\x02k\x1D_\x14a\0\x82W\x80cv,\x1D\x9B\x14a\0\xACW\x80c\x8E}i5\x14a\0\xC1W[`\0\x80\xFD[`\x02T`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xBFa\0\xBA6`\x04a\t~V[a\x01>V[\0[a\0\xD4a\0\xCF6`\x04a\t\xD9V[a\x02\x0FV[`@Q\x90\x81R` \x01a\0\xA3V[`\x01T`\x01`\xA0\x1B\x90\x04b\xFF\xFF\xFF\x16`@Qb\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0\xA3V[a\0\x8Fa\x01\x136`\x04a\t\xF2V[a\x026V[a\0\xBFa\x01&6`\x04a\t~V[a\x02\xD7V[a\0\xBFa\x0196`\x04a\n\x0FV[a\x03QV[`\0\x81\x15a\x01LW\x81a\x01UV[a\x01U\x85a\x026V[`\0T\x90\x91P`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x86\x16\x03a\x01\x8EW`\x02Ta\x01\x88\x90\x87\x90`\x01`\x01`\xA0\x1B\x03\x16\x86\x86\x85a\x07\x03V[Pa\x02\x08V[`\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x86\x16\x03a\x01\xBEW`\x02Ta\x01\x88\x90\x87\x90`\x01`\x01`\xA0\x1B\x03\x16\x86\x86\x85a\x07\xB7V[`\x02T`\0T`\x01T`@Qc\x94`M\xE9`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x82\x01R\x91\x83\x16`$\x83\x01R\x82\x16`D\x82\x01R\x90\x86\x16`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[PPPPPV[`\x01T`\0\x90a\x020\x90\x83\x90`\x01`\xA0\x1B\x90\x04b\xFF\xFF\xFF\x16b\x0FB@a\x07\xD2V[\x92\x91PPV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x83\x16\x03a\x02]Wa\x020d\x01\0\x02v\xA3`\x01a\n\xA8V[`\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x83\x16\x03a\x02\x92Wa\x020`\x01s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&a\n\xC7V[`\x02T`\0T`\x01T`@Qc\x94`M\xE9`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x82\x01R\x91\x83\x16`$\x83\x01R\x82\x16`D\x82\x01R\x90\x83\x16`d\x82\x01R`\x84\x01a\x01\xFFV[`\0\x81\x15a\x02\xE5W\x81a\x02\xEEV[a\x02\xEE\x85a\x026V[`\0T\x90\x91P`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x86\x16\x03a\x03!W`\x02Ta\x01\x88\x90\x87\x90`\x01`\x01`\xA0\x1B\x03\x16\x86\x86\x85a\x08\xC2V[`\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x86\x16\x03a\x01\xBEW`\x02Ta\x01\x88\x90\x87\x90`\x01`\x01`\xA0\x1B\x03\x16\x86\x86\x85a\x08\xDDV[`\0a\x03_\x82\x84\x01\x84a\t\xF2V[\x90Pa\x03\x80`@Q\x80`\x80\x01`@R\x80`H\x81R` \x01a\x0C\xF3`H\x919PV[a\x03\xAB`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01kamount0Delta`\xA0\x1B\x81RPPV[a\x03\xD6`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01kamount1Delta`\xA0\x1B\x81RPPV[a\x04\xC3`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e\x07F\xF6\xB6V\xE3`\xD4\x1B\x81RP3`\x01`\x01`\xA0\x1B\x03\x16c\r\xFE\x16\x81`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x046W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04Z\x91\x90a\n\xE6V[`\x01`\x01`\xA0\x1B\x03\x16c\x95\xD8\x9BA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x97W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x04\xBF\x91\x90\x81\x01\x90a\x0B=V[PPV[a\x05#`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01etoken1`\xD0\x1B\x81RP3`\x01`\x01`\xA0\x1B\x03\x16c\xD2\x12 \xA7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x046W=`\0\x80>=`\0\xFD[`@\x80Q\x86\x81R` \x81\x01\x86\x90R\x7F\xD4\x82A\xDFJu\xE6c\xB2\x9EU\xF9Pk1\xF7~\xD0\xF4\x8C\xFE~v\x12\xD1\x161D\x99]\xC1\xCA\x91\x01`@Q\x80\x91\x03\x90\xA1`\0\x85\x13\x15a\x06CW3`\x01`\x01`\xA0\x1B\x03\x16c\r\xFE\x16\x81`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xA3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xC7\x91\x90a\n\xE6V[`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R3`$\x83\x01R`D\x82\x01\x88\x90R\x91\x90\x91\x16\x90c#\xB8r\xDD\x90`d\x01[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x06\x1FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\x88\x91\x90a\x0B\xF2V[`\0\x84\x13\x15a\x06\xEBW3`\x01`\x01`\xA0\x1B\x03\x16c\xD2\x12 \xA7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x8AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xAE\x91\x90a\n\xE6V[`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R3`$\x83\x01R`D\x82\x01\x87\x90R\x91\x90\x91\x16\x90c#\xB8r\xDD\x90`d\x01a\x06\0V[\x84\x15\x80\x15a\x06\xF7WP\x83\x15[a\x02\x08Wa\x02\x08a\x0C\x14V[\x83`\x01`\x01`\xA0\x1B\x03\x16c\x12\x8A\xCB\x08\x83`\x01a\x07\x1E\x87a\x08\xFBV[a\x07'\x90a\x0C*V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x8C\x16` \x82\x01R\x87\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07l\x95\x94\x93\x92\x91\x90a\x0CrV[`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x07\x8AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xAE\x91\x90a\x0C\xB8V[PPPPPPPV[\x83`\x01`\x01`\xA0\x1B\x03\x16c\x12\x8A\xCB\x08\x83`\0a\x07\x1E\x87a\x08\xFBV[`\0\x80\x80`\0\x19\x85\x87\t\x85\x87\x02\x92P\x82\x81\x10\x83\x82\x03\x03\x91PP\x80`\0\x03a\x08\x0CW\x83\x82\x81a\x08\x02Wa\x08\x02a\x0C\xDCV[\x04\x92PPPa\x08\xBBV[\x80\x84\x11a\x08SW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtMath: mulDiv overflow`X\x1B`D\x82\x01R`d\x01a\x01\xFFV[`\0\x84\x86\x88\t\x85\x19`\x01\x90\x81\x01\x87\x16\x96\x87\x90\x04\x96\x82\x86\x03\x81\x90\x04\x95\x90\x92\x11\x90\x93\x03`\0\x82\x90\x03\x91\x90\x91\x04\x90\x92\x01\x91\x90\x91\x02\x91\x90\x91\x17`\x03\x84\x02`\x02\x90\x81\x18\x80\x86\x02\x82\x03\x02\x80\x86\x02\x82\x03\x02\x80\x86\x02\x82\x03\x02\x80\x86\x02\x82\x03\x02\x80\x86\x02\x82\x03\x02\x80\x86\x02\x90\x91\x03\x02\x02\x91PP[\x93\x92PPPV[\x83`\x01`\x01`\xA0\x1B\x03\x16c\x12\x8A\xCB\x08\x83`\x01a\x07'\x87a\x08\xFBV[\x83`\x01`\x01`\xA0\x1B\x03\x16c\x12\x8A\xCB\x08\x83`\0a\x07'\x87a\x08\xFBV[PV[`\0`\x01`\x01`\xFF\x1B\x03\x82\x11\x15a\teW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FSafeCast: value doesn't fit in a`D\x82\x01Rg7\x104\xB7:\x19\x1A\x9B`\xC1\x1B`d\x82\x01R`\x84\x01a\x01\xFFV[P\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x08\xF8W`\0\x80\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\t\x96W`\0\x80\xFD[\x855a\t\xA1\x81a\tiV[\x94P` \x86\x015a\t\xB1\x81a\tiV[\x93P`@\x86\x015\x92P``\x86\x015a\t\xC8\x81a\tiV[\x94\x97\x93\x96P\x91\x94`\x80\x015\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\t\xEBW`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\n\x04W`\0\x80\xFD[\x815a\x08\xBB\x81a\tiV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\n%W`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\nJW`\0\x80\xFD[\x85\x01`\x1F\x81\x01\x87\x13a\n[W`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\nrW`\0\x80\xFD[\x87` \x82\x84\x01\x01\x11\x15a\n\x84W`\0\x80\xFD[\x94\x97\x93\x96P` \x01\x94PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x020Wa\x020a\n\x92V[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x020Wa\x020a\n\x92V[`\0` \x82\x84\x03\x12\x15a\n\xF8W`\0\x80\xFD[\x81Qa\x08\xBB\x81a\tiV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0[\x83\x81\x10\x15a\x0B4W\x81\x81\x01Q\x83\x82\x01R` \x01a\x0B\x1CV[PP`\0\x91\x01RV[`\0` \x82\x84\x03\x12\x15a\x0BOW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0BfW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x0BwW`\0\x80\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\x91Wa\x0B\x91a\x0B\x03V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0B\xC0Wa\x0B\xC0a\x0B\x03V[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a\x0B\xD8W`\0\x80\xFD[a\x0B\xE9\x82` \x83\x01` \x86\x01a\x0B\x19V[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a\x0C\x04W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x08\xBBW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[`\0`\x01`\xFF\x1B\x82\x01a\x0C?Wa\x0C?a\n\x92V[P`\0\x03\x90V[`\0\x81Q\x80\x84Ra\x0C^\x81` \x86\x01` \x86\x01a\x0B\x19V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x15\x15` \x83\x01R`@\x82\x01\x85\x90R\x83\x16``\x82\x01R`\xA0`\x80\x82\x01\x81\x90R`\0\x90a\x0C\xAD\x90\x83\x01\x84a\x0CFV[\x97\x96PPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0C\xCBW`\0\x80\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD\xFE-------------------------uniswapV3SwapCallback--------------------------\xA2dipfsX\"\x12 F\xD9_\xD2\xA2J\xCE^\xA2u\xAD\xA8.xWQ6\xBC\x19mQiAy`\x94YD.Q\x16&dsolcC\0\x08\x1A\x003";
    /// The bytecode of the contract.
    pub static DEXUNISWAPV3_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0}W`\x005`\xE0\x1C\x80c\xBF\\\x98\"\x11a\0[W\x80c\xBF\\\x98\"\x14a\0\xE2W\x80c\xC1\x84\xA2u\x14a\x01\x05W\x80c\xD6-\x9E\xA3\x14a\x01\x18W\x80c\xFAF\x1E3\x14a\x01+W`\0\x80\xFD[\x80c\x02k\x1D_\x14a\0\x82W\x80cv,\x1D\x9B\x14a\0\xACW\x80c\x8E}i5\x14a\0\xC1W[`\0\x80\xFD[`\x02T`\x01`\x01`\xA0\x1B\x03\x16[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xBFa\0\xBA6`\x04a\t~V[a\x01>V[\0[a\0\xD4a\0\xCF6`\x04a\t\xD9V[a\x02\x0FV[`@Q\x90\x81R` \x01a\0\xA3V[`\x01T`\x01`\xA0\x1B\x90\x04b\xFF\xFF\xFF\x16`@Qb\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\0\xA3V[a\0\x8Fa\x01\x136`\x04a\t\xF2V[a\x026V[a\0\xBFa\x01&6`\x04a\t~V[a\x02\xD7V[a\0\xBFa\x0196`\x04a\n\x0FV[a\x03QV[`\0\x81\x15a\x01LW\x81a\x01UV[a\x01U\x85a\x026V[`\0T\x90\x91P`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x86\x16\x03a\x01\x8EW`\x02Ta\x01\x88\x90\x87\x90`\x01`\x01`\xA0\x1B\x03\x16\x86\x86\x85a\x07\x03V[Pa\x02\x08V[`\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x86\x16\x03a\x01\xBEW`\x02Ta\x01\x88\x90\x87\x90`\x01`\x01`\xA0\x1B\x03\x16\x86\x86\x85a\x07\xB7V[`\x02T`\0T`\x01T`@Qc\x94`M\xE9`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x82\x01R\x91\x83\x16`$\x83\x01R\x82\x16`D\x82\x01R\x90\x86\x16`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[PPPPPV[`\x01T`\0\x90a\x020\x90\x83\x90`\x01`\xA0\x1B\x90\x04b\xFF\xFF\xFF\x16b\x0FB@a\x07\xD2V[\x92\x91PPV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x83\x16\x03a\x02]Wa\x020d\x01\0\x02v\xA3`\x01a\n\xA8V[`\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x83\x16\x03a\x02\x92Wa\x020`\x01s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&a\n\xC7V[`\x02T`\0T`\x01T`@Qc\x94`M\xE9`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x82\x01R\x91\x83\x16`$\x83\x01R\x82\x16`D\x82\x01R\x90\x83\x16`d\x82\x01R`\x84\x01a\x01\xFFV[`\0\x81\x15a\x02\xE5W\x81a\x02\xEEV[a\x02\xEE\x85a\x026V[`\0T\x90\x91P`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x86\x16\x03a\x03!W`\x02Ta\x01\x88\x90\x87\x90`\x01`\x01`\xA0\x1B\x03\x16\x86\x86\x85a\x08\xC2V[`\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90\x86\x16\x03a\x01\xBEW`\x02Ta\x01\x88\x90\x87\x90`\x01`\x01`\xA0\x1B\x03\x16\x86\x86\x85a\x08\xDDV[`\0a\x03_\x82\x84\x01\x84a\t\xF2V[\x90Pa\x03\x80`@Q\x80`\x80\x01`@R\x80`H\x81R` \x01a\x0C\xF3`H\x919PV[a\x03\xAB`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01kamount0Delta`\xA0\x1B\x81RPPV[a\x03\xD6`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01kamount1Delta`\xA0\x1B\x81RPPV[a\x04\xC3`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01e\x07F\xF6\xB6V\xE3`\xD4\x1B\x81RP3`\x01`\x01`\xA0\x1B\x03\x16c\r\xFE\x16\x81`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x046W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04Z\x91\x90a\n\xE6V[`\x01`\x01`\xA0\x1B\x03\x16c\x95\xD8\x9BA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x97W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x04\xBF\x91\x90\x81\x01\x90a\x0B=V[PPV[a\x05#`@Q\x80`@\x01`@R\x80`\x06\x81R` \x01etoken1`\xD0\x1B\x81RP3`\x01`\x01`\xA0\x1B\x03\x16c\xD2\x12 \xA7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x046W=`\0\x80>=`\0\xFD[`@\x80Q\x86\x81R` \x81\x01\x86\x90R\x7F\xD4\x82A\xDFJu\xE6c\xB2\x9EU\xF9Pk1\xF7~\xD0\xF4\x8C\xFE~v\x12\xD1\x161D\x99]\xC1\xCA\x91\x01`@Q\x80\x91\x03\x90\xA1`\0\x85\x13\x15a\x06CW3`\x01`\x01`\xA0\x1B\x03\x16c\r\xFE\x16\x81`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xA3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xC7\x91\x90a\n\xE6V[`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R3`$\x83\x01R`D\x82\x01\x88\x90R\x91\x90\x91\x16\x90c#\xB8r\xDD\x90`d\x01[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x06\x1FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\x88\x91\x90a\x0B\xF2V[`\0\x84\x13\x15a\x06\xEBW3`\x01`\x01`\xA0\x1B\x03\x16c\xD2\x12 \xA7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x8AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xAE\x91\x90a\n\xE6V[`@Qc#\xB8r\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R3`$\x83\x01R`D\x82\x01\x87\x90R\x91\x90\x91\x16\x90c#\xB8r\xDD\x90`d\x01a\x06\0V[\x84\x15\x80\x15a\x06\xF7WP\x83\x15[a\x02\x08Wa\x02\x08a\x0C\x14V[\x83`\x01`\x01`\xA0\x1B\x03\x16c\x12\x8A\xCB\x08\x83`\x01a\x07\x1E\x87a\x08\xFBV[a\x07'\x90a\x0C*V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x8C\x16` \x82\x01R\x87\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07l\x95\x94\x93\x92\x91\x90a\x0CrV[`@\x80Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x07\x8AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xAE\x91\x90a\x0C\xB8V[PPPPPPPV[\x83`\x01`\x01`\xA0\x1B\x03\x16c\x12\x8A\xCB\x08\x83`\0a\x07\x1E\x87a\x08\xFBV[`\0\x80\x80`\0\x19\x85\x87\t\x85\x87\x02\x92P\x82\x81\x10\x83\x82\x03\x03\x91PP\x80`\0\x03a\x08\x0CW\x83\x82\x81a\x08\x02Wa\x08\x02a\x0C\xDCV[\x04\x92PPPa\x08\xBBV[\x80\x84\x11a\x08SW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtMath: mulDiv overflow`X\x1B`D\x82\x01R`d\x01a\x01\xFFV[`\0\x84\x86\x88\t\x85\x19`\x01\x90\x81\x01\x87\x16\x96\x87\x90\x04\x96\x82\x86\x03\x81\x90\x04\x95\x90\x92\x11\x90\x93\x03`\0\x82\x90\x03\x91\x90\x91\x04\x90\x92\x01\x91\x90\x91\x02\x91\x90\x91\x17`\x03\x84\x02`\x02\x90\x81\x18\x80\x86\x02\x82\x03\x02\x80\x86\x02\x82\x03\x02\x80\x86\x02\x82\x03\x02\x80\x86\x02\x82\x03\x02\x80\x86\x02\x82\x03\x02\x80\x86\x02\x90\x91\x03\x02\x02\x91PP[\x93\x92PPPV[\x83`\x01`\x01`\xA0\x1B\x03\x16c\x12\x8A\xCB\x08\x83`\x01a\x07'\x87a\x08\xFBV[\x83`\x01`\x01`\xA0\x1B\x03\x16c\x12\x8A\xCB\x08\x83`\0a\x07'\x87a\x08\xFBV[PV[`\0`\x01`\x01`\xFF\x1B\x03\x82\x11\x15a\teW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FSafeCast: value doesn't fit in a`D\x82\x01Rg7\x104\xB7:\x19\x1A\x9B`\xC1\x1B`d\x82\x01R`\x84\x01a\x01\xFFV[P\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x08\xF8W`\0\x80\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\t\x96W`\0\x80\xFD[\x855a\t\xA1\x81a\tiV[\x94P` \x86\x015a\t\xB1\x81a\tiV[\x93P`@\x86\x015\x92P``\x86\x015a\t\xC8\x81a\tiV[\x94\x97\x93\x96P\x91\x94`\x80\x015\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\t\xEBW`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\n\x04W`\0\x80\xFD[\x815a\x08\xBB\x81a\tiV[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\n%W`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\nJW`\0\x80\xFD[\x85\x01`\x1F\x81\x01\x87\x13a\n[W`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\nrW`\0\x80\xFD[\x87` \x82\x84\x01\x01\x11\x15a\n\x84W`\0\x80\xFD[\x94\x97\x93\x96P` \x01\x94PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x020Wa\x020a\n\x92V[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x020Wa\x020a\n\x92V[`\0` \x82\x84\x03\x12\x15a\n\xF8W`\0\x80\xFD[\x81Qa\x08\xBB\x81a\tiV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0[\x83\x81\x10\x15a\x0B4W\x81\x81\x01Q\x83\x82\x01R` \x01a\x0B\x1CV[PP`\0\x91\x01RV[`\0` \x82\x84\x03\x12\x15a\x0BOW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0BfW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x0BwW`\0\x80\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\x91Wa\x0B\x91a\x0B\x03V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0B\xC0Wa\x0B\xC0a\x0B\x03V[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a\x0B\xD8W`\0\x80\xFD[a\x0B\xE9\x82` \x83\x01` \x86\x01a\x0B\x19V[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a\x0C\x04W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x08\xBBW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[`\0`\x01`\xFF\x1B\x82\x01a\x0C?Wa\x0C?a\n\x92V[P`\0\x03\x90V[`\0\x81Q\x80\x84Ra\x0C^\x81` \x86\x01` \x86\x01a\x0B\x19V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x15\x15` \x83\x01R`@\x82\x01\x85\x90R\x83\x16``\x82\x01R`\xA0`\x80\x82\x01\x81\x90R`\0\x90a\x0C\xAD\x90\x83\x01\x84a\x0CFV[\x97\x96PPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0C\xCBW`\0\x80\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD\xFE-------------------------uniswapV3SwapCallback--------------------------\xA2dipfsX\"\x12 F\xD9_\xD2\xA2J\xCE^\xA2u\xAD\xA8.xWQ6\xBC\x19mQiAy`\x94YD.Q\x16&dsolcC\0\x08\x1A\x003";
    /// The deployed bytecode of the contract.
    pub static DEXUNISWAPV3_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct DexUniswapV3<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DexUniswapV3<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for DexUniswapV3<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for DexUniswapV3<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for DexUniswapV3<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(DexUniswapV3))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DexUniswapV3<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    DEXUNISWAPV3_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                DEXUNISWAPV3_ABI.clone(),
                DEXUNISWAPV3_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getFeeAmount` (0xbf5c9822) function
        pub fn get_fee_amount(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([191, 92, 152, 34], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPool` (0x026b1d5f) function
        pub fn get_pool(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([2, 107, 29, 95], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSqrtPriceLimitX96` (0xc184a275) function
        pub fn get_sqrt_price_limit_x96(
            &self,
            token_in: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([193, 132, 162, 117], token_in)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSwapFee` (0x8e7d6935) function
        pub fn get_swap_fee(
            &self,
            amount_in: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([142, 125, 105, 53], amount_in)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapExactIn` (0xd62d9ea3) function
        pub fn swap_exact_in(
            &self,
            from: ::ethers::core::types::Address,
            token_in: ::ethers::core::types::Address,
            amount_in: ::ethers::core::types::U256,
            to: ::ethers::core::types::Address,
            sqrt_price_limit_x96: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [214, 45, 158, 163],
                    (from, token_in, amount_in, to, sqrt_price_limit_x96),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapExactOut` (0x762c1d9b) function
        pub fn swap_exact_out(
            &self,
            from: ::ethers::core::types::Address,
            token_in: ::ethers::core::types::Address,
            amount_out: ::ethers::core::types::U256,
            to: ::ethers::core::types::Address,
            sqrt_price_limit_x96: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [118, 44, 29, 155],
                    (from, token_in, amount_out, to, sqrt_price_limit_x96),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `uniswapV3SwapCallback` (0xfa461e33) function
        pub fn uniswap_v3_swap_callback(
            &self,
            amount_0_delta: ::ethers::core::types::I256,
            amount_1_delta: ::ethers::core::types::I256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 70, 30, 51], (amount_0_delta, amount_1_delta, data))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `SwapCallback` event
        pub fn swap_callback_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SwapCallbackFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SwapCallbackFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for DexUniswapV3<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `TokenCanNotSwapWithSelf` with signature `TokenCanNotSwapWithSelf(address)` and selector `0xecfbebe4`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "TokenCanNotSwapWithSelf",
        abi = "TokenCanNotSwapWithSelf(address)"
    )]
    pub struct TokenCanNotSwapWithSelf {
        pub token: ::ethers::core::types::Address,
    }
    ///Custom Error type `TokenNotMatch` with signature `TokenNotMatch(address,address,address,address)` and selector `0x94604de9`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "TokenNotMatch",
        abi = "TokenNotMatch(address,address,address,address)"
    )]
    pub struct TokenNotMatch {
        pub pool: ::ethers::core::types::Address,
        pub token_0: ::ethers::core::types::Address,
        pub token_1: ::ethers::core::types::Address,
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's custom errors
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum DexUniswapV3Errors {
        TokenCanNotSwapWithSelf(TokenCanNotSwapWithSelf),
        TokenNotMatch(TokenNotMatch),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for DexUniswapV3Errors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <TokenCanNotSwapWithSelf as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenCanNotSwapWithSelf(decoded));
            }
            if let Ok(decoded) = <TokenNotMatch as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenNotMatch(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DexUniswapV3Errors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::TokenCanNotSwapWithSelf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenNotMatch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for DexUniswapV3Errors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <TokenCanNotSwapWithSelf as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TokenNotMatch as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for DexUniswapV3Errors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::TokenCanNotSwapWithSelf(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TokenNotMatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for DexUniswapV3Errors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<TokenCanNotSwapWithSelf> for DexUniswapV3Errors {
        fn from(value: TokenCanNotSwapWithSelf) -> Self {
            Self::TokenCanNotSwapWithSelf(value)
        }
    }
    impl ::core::convert::From<TokenNotMatch> for DexUniswapV3Errors {
        fn from(value: TokenNotMatch) -> Self {
            Self::TokenNotMatch(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "SwapCallback", abi = "SwapCallback(int256,int256)")]
    pub struct SwapCallbackFilter {
        pub amount_0_delta: ::ethers::core::types::I256,
        pub amount_1_delta: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `getFeeAmount` function with signature `getFeeAmount()` and selector `0xbf5c9822`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getFeeAmount", abi = "getFeeAmount()")]
    pub struct GetFeeAmountCall;
    ///Container type for all input parameters for the `getPool` function with signature `getPool()` and selector `0x026b1d5f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getPool", abi = "getPool()")]
    pub struct GetPoolCall;
    ///Container type for all input parameters for the `getSqrtPriceLimitX96` function with signature `getSqrtPriceLimitX96(address)` and selector `0xc184a275`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getSqrtPriceLimitX96", abi = "getSqrtPriceLimitX96(address)")]
    pub struct GetSqrtPriceLimitX96Call {
        pub token_in: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getSwapFee` function with signature `getSwapFee(uint256)` and selector `0x8e7d6935`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getSwapFee", abi = "getSwapFee(uint256)")]
    pub struct GetSwapFeeCall {
        pub amount_in: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `swapExactIn` function with signature `swapExactIn(address,address,uint256,address,uint256)` and selector `0xd62d9ea3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "swapExactIn",
        abi = "swapExactIn(address,address,uint256,address,uint256)"
    )]
    pub struct SwapExactInCall {
        pub from: ::ethers::core::types::Address,
        pub token_in: ::ethers::core::types::Address,
        pub amount_in: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
        pub sqrt_price_limit_x96: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `swapExactOut` function with signature `swapExactOut(address,address,uint256,address,uint256)` and selector `0x762c1d9b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "swapExactOut",
        abi = "swapExactOut(address,address,uint256,address,uint256)"
    )]
    pub struct SwapExactOutCall {
        pub from: ::ethers::core::types::Address,
        pub token_in: ::ethers::core::types::Address,
        pub amount_out: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
        pub sqrt_price_limit_x96: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `uniswapV3SwapCallback` function with signature `uniswapV3SwapCallback(int256,int256,bytes)` and selector `0xfa461e33`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "uniswapV3SwapCallback",
        abi = "uniswapV3SwapCallback(int256,int256,bytes)"
    )]
    pub struct UniswapV3SwapCallbackCall {
        pub amount_0_delta: ::ethers::core::types::I256,
        pub amount_1_delta: ::ethers::core::types::I256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum DexUniswapV3Calls {
        GetFeeAmount(GetFeeAmountCall),
        GetPool(GetPoolCall),
        GetSqrtPriceLimitX96(GetSqrtPriceLimitX96Call),
        GetSwapFee(GetSwapFeeCall),
        SwapExactIn(SwapExactInCall),
        SwapExactOut(SwapExactOutCall),
        UniswapV3SwapCallback(UniswapV3SwapCallbackCall),
    }
    impl ::ethers::core::abi::AbiDecode for DexUniswapV3Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetFeeAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetFeeAmount(decoded));
            }
            if let Ok(decoded) = <GetPoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPool(decoded));
            }
            if let Ok(decoded) = <GetSqrtPriceLimitX96Call as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetSqrtPriceLimitX96(decoded));
            }
            if let Ok(decoded) = <GetSwapFeeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetSwapFee(decoded));
            }
            if let Ok(decoded) = <SwapExactInCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SwapExactIn(decoded));
            }
            if let Ok(decoded) = <SwapExactOutCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SwapExactOut(decoded));
            }
            if let Ok(decoded) = <UniswapV3SwapCallbackCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::UniswapV3SwapCallback(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DexUniswapV3Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetFeeAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSqrtPriceLimitX96(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSwapFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapExactIn(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapExactOut(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UniswapV3SwapCallback(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for DexUniswapV3Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetFeeAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSqrtPriceLimitX96(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetSwapFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapExactIn(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapExactOut(element) => ::core::fmt::Display::fmt(element, f),
                Self::UniswapV3SwapCallback(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<GetFeeAmountCall> for DexUniswapV3Calls {
        fn from(value: GetFeeAmountCall) -> Self {
            Self::GetFeeAmount(value)
        }
    }
    impl ::core::convert::From<GetPoolCall> for DexUniswapV3Calls {
        fn from(value: GetPoolCall) -> Self {
            Self::GetPool(value)
        }
    }
    impl ::core::convert::From<GetSqrtPriceLimitX96Call> for DexUniswapV3Calls {
        fn from(value: GetSqrtPriceLimitX96Call) -> Self {
            Self::GetSqrtPriceLimitX96(value)
        }
    }
    impl ::core::convert::From<GetSwapFeeCall> for DexUniswapV3Calls {
        fn from(value: GetSwapFeeCall) -> Self {
            Self::GetSwapFee(value)
        }
    }
    impl ::core::convert::From<SwapExactInCall> for DexUniswapV3Calls {
        fn from(value: SwapExactInCall) -> Self {
            Self::SwapExactIn(value)
        }
    }
    impl ::core::convert::From<SwapExactOutCall> for DexUniswapV3Calls {
        fn from(value: SwapExactOutCall) -> Self {
            Self::SwapExactOut(value)
        }
    }
    impl ::core::convert::From<UniswapV3SwapCallbackCall> for DexUniswapV3Calls {
        fn from(value: UniswapV3SwapCallbackCall) -> Self {
            Self::UniswapV3SwapCallback(value)
        }
    }
    ///Container type for all return fields from the `getFeeAmount` function with signature `getFeeAmount()` and selector `0xbf5c9822`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetFeeAmountReturn(pub u32);
    ///Container type for all return fields from the `getPool` function with signature `getPool()` and selector `0x026b1d5f`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetPoolReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getSqrtPriceLimitX96` function with signature `getSqrtPriceLimitX96(address)` and selector `0xc184a275`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetSqrtPriceLimitX96Return(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getSwapFee` function with signature `getSwapFee(uint256)` and selector `0x8e7d6935`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetSwapFeeReturn(pub ::ethers::core::types::U256);
}