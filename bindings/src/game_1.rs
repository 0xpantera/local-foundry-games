pub use game_1::*;
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
pub mod game_1 {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"winner\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Winner\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"win\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static GAME1_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
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
        96,
        15,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        96,
        160,
        128,
        97,
        0,
        30,
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
        96,
        15,
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
        96,
        40,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        71,
        60,
        169,
        108,
        20,
        96,
        45,
        87,
        91,
        96,
        0,
        128,
        253,
        91,
        96,
        51,
        96,
        53,
        86,
        91,
        0,
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
        19,
        40,
        30,
        149,
        77,
        109,
        225,
        198,
        212,
        222,
        136,
        81,
        103,
        82,
        167,
        78,
        212,
        202,
        2,
        230,
        243,
        0,
        17,
        74,
        242,
        76,
        177,
        208,
        204,
        237,
        14,
        74,
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
    pub static GAME1_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
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
        96,
        15,
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
        96,
        40,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        71,
        60,
        169,
        108,
        20,
        96,
        45,
        87,
        91,
        96,
        0,
        128,
        253,
        91,
        96,
        51,
        96,
        53,
        86,
        91,
        0,
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
        19,
        40,
        30,
        149,
        77,
        109,
        225,
        198,
        212,
        222,
        136,
        81,
        103,
        82,
        167,
        78,
        212,
        202,
        2,
        230,
        243,
        0,
        17,
        74,
        242,
        76,
        177,
        208,
        204,
        237,
        14,
        74,
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
    pub static GAME1_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Game1<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Game1<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Game1<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Game1<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Game1<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(Game1)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Game1<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    GAME1_ABI.clone(),
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
                GAME1_ABI.clone(),
                GAME1_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `win` (0x473ca96c) function
        pub fn win(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([71, 60, 169, 108], ())
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
    for Game1<M> {
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
}
