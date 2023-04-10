pub use game_2::*;
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
pub mod game_2 {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"winner\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Winner\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_x\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setX\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_y\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setY\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"win\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"x\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"y\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static GAME2_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = &[
        96,
        128,
        96,
        64,
        82,
        52,
        128,
        21,
        97,
        0,
        16,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        97,
        1,
        147,
        128,
        97,
        0,
        32,
        96,
        0,
        57,
        96,
        0,
        243,
        254,
        96,
        128,
        96,
        64,
        82,
        52,
        128,
        21,
        97,
        0,
        16,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        96,
        4,
        54,
        16,
        97,
        0,
        87,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        12,
        85,
        105,
        156,
        20,
        97,
        0,
        92,
        87,
        128,
        99,
        64,
        24,
        217,
        170,
        20,
        97,
        0,
        119,
        87,
        128,
        99,
        71,
        60,
        169,
        108,
        20,
        97,
        0,
        140,
        87,
        128,
        99,
        104,
        212,
        102,
        184,
        20,
        97,
        0,
        148,
        87,
        128,
        99,
        165,
        109,
        254,
        74,
        20,
        97,
        0,
        167,
        87,
        91,
        96,
        0,
        128,
        253,
        91,
        97,
        0,
        101,
        96,
        0,
        84,
        129,
        86,
        91,
        96,
        64,
        81,
        144,
        129,
        82,
        96,
        32,
        1,
        96,
        64,
        81,
        128,
        145,
        3,
        144,
        243,
        91,
        97,
        0,
        138,
        97,
        0,
        133,
        54,
        96,
        4,
        97,
        1,
        29,
        86,
        91,
        96,
        0,
        85,
        86,
        91,
        0,
        91,
        97,
        0,
        138,
        97,
        0,
        176,
        86,
        91,
        97,
        0,
        138,
        97,
        0,
        162,
        54,
        96,
        4,
        97,
        1,
        29,
        86,
        91,
        96,
        1,
        85,
        86,
        91,
        97,
        0,
        101,
        96,
        1,
        84,
        129,
        86,
        91,
        96,
        0,
        128,
        84,
        17,
        128,
        21,
        97,
        0,
        195,
        87,
        80,
        96,
        0,
        96,
        1,
        84,
        17,
        91,
        97,
        0,
        204,
        87,
        96,
        0,
        128,
        253,
        91,
        96,
        1,
        84,
        96,
        0,
        84,
        97,
        0,
        220,
        145,
        144,
        97,
        1,
        54,
        86,
        91,
        96,
        50,
        20,
        97,
        0,
        232,
        87,
        96,
        0,
        128,
        253,
        91,
        96,
        64,
        81,
        51,
        129,
        82,
        127,
        116,
        92,
        144,
        182,
        86,
        180,
        170,
        254,
        41,
        108,
        140,
        163,
        90,
        234,
        207,
        229,
        108,
        185,
        108,
        144,
        225,
        211,
        32,
        229,
        218,
        100,
        63,
        255,
        16,
        81,
        182,
        192,
        144,
        96,
        32,
        1,
        96,
        64,
        81,
        128,
        145,
        3,
        144,
        161,
        86,
        91,
        96,
        0,
        96,
        32,
        130,
        132,
        3,
        18,
        21,
        97,
        1,
        47,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        53,
        145,
        144,
        80,
        86,
        91,
        128,
        130,
        1,
        128,
        130,
        17,
        21,
        97,
        1,
        87,
        87,
        99,
        78,
        72,
        123,
        113,
        96,
        224,
        27,
        96,
        0,
        82,
        96,
        17,
        96,
        4,
        82,
        96,
        36,
        96,
        0,
        253,
        91,
        146,
        145,
        80,
        80,
        86,
        254,
        162,
        100,
        105,
        112,
        102,
        115,
        88,
        34,
        18,
        32,
        212,
        252,
        210,
        99,
        74,
        1,
        225,
        124,
        67,
        110,
        125,
        193,
        124,
        249,
        234,
        129,
        211,
        31,
        184,
        107,
        174,
        184,
        68,
        30,
        62,
        3,
        78,
        170,
        70,
        200,
        249,
        17,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        19,
        0,
        51,
    ];
    ///The bytecode of the contract.
    pub static GAME2_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = &[
        96,
        128,
        96,
        64,
        82,
        52,
        128,
        21,
        97,
        0,
        16,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        96,
        4,
        54,
        16,
        97,
        0,
        87,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        12,
        85,
        105,
        156,
        20,
        97,
        0,
        92,
        87,
        128,
        99,
        64,
        24,
        217,
        170,
        20,
        97,
        0,
        119,
        87,
        128,
        99,
        71,
        60,
        169,
        108,
        20,
        97,
        0,
        140,
        87,
        128,
        99,
        104,
        212,
        102,
        184,
        20,
        97,
        0,
        148,
        87,
        128,
        99,
        165,
        109,
        254,
        74,
        20,
        97,
        0,
        167,
        87,
        91,
        96,
        0,
        128,
        253,
        91,
        97,
        0,
        101,
        96,
        0,
        84,
        129,
        86,
        91,
        96,
        64,
        81,
        144,
        129,
        82,
        96,
        32,
        1,
        96,
        64,
        81,
        128,
        145,
        3,
        144,
        243,
        91,
        97,
        0,
        138,
        97,
        0,
        133,
        54,
        96,
        4,
        97,
        1,
        29,
        86,
        91,
        96,
        0,
        85,
        86,
        91,
        0,
        91,
        97,
        0,
        138,
        97,
        0,
        176,
        86,
        91,
        97,
        0,
        138,
        97,
        0,
        162,
        54,
        96,
        4,
        97,
        1,
        29,
        86,
        91,
        96,
        1,
        85,
        86,
        91,
        97,
        0,
        101,
        96,
        1,
        84,
        129,
        86,
        91,
        96,
        0,
        128,
        84,
        17,
        128,
        21,
        97,
        0,
        195,
        87,
        80,
        96,
        0,
        96,
        1,
        84,
        17,
        91,
        97,
        0,
        204,
        87,
        96,
        0,
        128,
        253,
        91,
        96,
        1,
        84,
        96,
        0,
        84,
        97,
        0,
        220,
        145,
        144,
        97,
        1,
        54,
        86,
        91,
        96,
        50,
        20,
        97,
        0,
        232,
        87,
        96,
        0,
        128,
        253,
        91,
        96,
        64,
        81,
        51,
        129,
        82,
        127,
        116,
        92,
        144,
        182,
        86,
        180,
        170,
        254,
        41,
        108,
        140,
        163,
        90,
        234,
        207,
        229,
        108,
        185,
        108,
        144,
        225,
        211,
        32,
        229,
        218,
        100,
        63,
        255,
        16,
        81,
        182,
        192,
        144,
        96,
        32,
        1,
        96,
        64,
        81,
        128,
        145,
        3,
        144,
        161,
        86,
        91,
        96,
        0,
        96,
        32,
        130,
        132,
        3,
        18,
        21,
        97,
        1,
        47,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        53,
        145,
        144,
        80,
        86,
        91,
        128,
        130,
        1,
        128,
        130,
        17,
        21,
        97,
        1,
        87,
        87,
        99,
        78,
        72,
        123,
        113,
        96,
        224,
        27,
        96,
        0,
        82,
        96,
        17,
        96,
        4,
        82,
        96,
        36,
        96,
        0,
        253,
        91,
        146,
        145,
        80,
        80,
        86,
        254,
        162,
        100,
        105,
        112,
        102,
        115,
        88,
        34,
        18,
        32,
        212,
        252,
        210,
        99,
        74,
        1,
        225,
        124,
        67,
        110,
        125,
        193,
        124,
        249,
        234,
        129,
        211,
        31,
        184,
        107,
        174,
        184,
        68,
        30,
        62,
        3,
        78,
        170,
        70,
        200,
        249,
        17,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        19,
        0,
        51,
    ];
    ///The deployed bytecode of the contract.
    pub static GAME2_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Game2<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Game2<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Game2<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Game2<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Game2<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(Game2)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Game2<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    GAME2_ABI.clone(),
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
                GAME2_ABI.clone(),
                GAME2_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `setX` (0x4018d9aa) function
        pub fn set_x(
            &self,
            x: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([64, 24, 217, 170], x)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setY` (0x68d466b8) function
        pub fn set_y(
            &self,
            y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([104, 212, 102, 184], y)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `win` (0x473ca96c) function
        pub fn win(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([71, 60, 169, 108], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `x` (0x0c55699c) function
        pub fn x(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([12, 85, 105, 156], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `y` (0xa56dfe4a) function
        pub fn y(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([165, 109, 254, 74], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Winner` event
        pub fn winner_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, WinnerFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, WinnerFilter> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Game2<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Winner", abi = "Winner(address)")]
    pub struct WinnerFilter {
        pub winner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setX` function with signature `setX(uint256)` and selector `0x4018d9aa`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setX", abi = "setX(uint256)")]
    pub struct SetXCall {
        pub x: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setY` function with signature `setY(uint256)` and selector `0x68d466b8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setY", abi = "setY(uint256)")]
    pub struct SetYCall {
        pub y: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `win` function with signature `win()` and selector `0x473ca96c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "win", abi = "win()")]
    pub struct WinCall;
    ///Container type for all input parameters for the `x` function with signature `x()` and selector `0x0c55699c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "x", abi = "x()")]
    pub struct XCall;
    ///Container type for all input parameters for the `y` function with signature `y()` and selector `0xa56dfe4a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "y", abi = "y()")]
    pub struct YCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum Game2Calls {
        SetX(SetXCall),
        SetY(SetYCall),
        Win(WinCall),
        X(XCall),
        Y(YCall),
    }
    impl ::ethers::core::abi::AbiDecode for Game2Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <SetXCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetX(decoded));
            }
            if let Ok(decoded)
                = <SetYCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetY(decoded));
            }
            if let Ok(decoded)
                = <WinCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Win(decoded));
            }
            if let Ok(decoded)
                = <XCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::X(decoded));
            }
            if let Ok(decoded)
                = <YCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Y(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for Game2Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::SetX(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetY(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Win(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::X(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Y(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for Game2Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::SetX(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetY(element) => ::core::fmt::Display::fmt(element, f),
                Self::Win(element) => ::core::fmt::Display::fmt(element, f),
                Self::X(element) => ::core::fmt::Display::fmt(element, f),
                Self::Y(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<SetXCall> for Game2Calls {
        fn from(value: SetXCall) -> Self {
            Self::SetX(value)
        }
    }
    impl ::core::convert::From<SetYCall> for Game2Calls {
        fn from(value: SetYCall) -> Self {
            Self::SetY(value)
        }
    }
    impl ::core::convert::From<WinCall> for Game2Calls {
        fn from(value: WinCall) -> Self {
            Self::Win(value)
        }
    }
    impl ::core::convert::From<XCall> for Game2Calls {
        fn from(value: XCall) -> Self {
            Self::X(value)
        }
    }
    impl ::core::convert::From<YCall> for Game2Calls {
        fn from(value: YCall) -> Self {
            Self::Y(value)
        }
    }
    ///Container type for all return fields from the `x` function with signature `x()` and selector `0x0c55699c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct XReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `y` function with signature `y()` and selector `0xa56dfe4a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct YReturn(pub ::ethers::core::types::U256);
}
