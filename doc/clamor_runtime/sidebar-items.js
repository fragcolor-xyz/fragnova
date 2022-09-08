window.SIDEBAR_ITEMS = {"constant":[["CENTS",""],["CONTRACTS_DEBUG_OUTPUT",""],["DAYS",""],["DOLLARS",""],["HOURS",""],["MILLICENTS",""],["MILLISECS_PER_BLOCK","This determines the average expected block time that we are targeting. Blocks will be produced at a minimum duration defined by `SLOT_DURATION`. `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked up by `pallet_aura` to implement `fn slot_duration()`."],["MINUTES",""],["SLOT_DURATION",""],["VERSION",""],["WASM_BINARY",""],["WASM_BINARY_BLOATY",""],["WEIGHT_PER_SECOND",""]],"enum":[["BalancesCall","Contains one variant per dispatchable that can be called by an extrinsic."],["Call",""],["Event",""],["OriginCaller",""],["ProtosCall","Contains one variant per dispatchable that can be called by an extrinsic."],["TimestampCall","Contains one variant per dispatchable that can be called by an extrinsic."]],"fn":[["deposit",""],["native_version","The version information used to identify this runtime when compiled natively."]],"macro":[["construct_runtime","Construct a runtime, with the given name and the given pallets."],["parameter_types","Create new implementations of the `Get` trait."]],"mod":[["api",""],["opaque","Opaque types. These are used by the CLI to instantiate machinery that don’t need to know the specifics of the runtime. They can then be made to be agnostic over specific formats of data like extrinsics, allowing for them to continue syncing the network through upgrades to even the core data structures."]],"struct":[["ApprovalDeposit",""],["AssetDeposit",""],["BlockExecutionWeight","Time to execute an empty block. Calculated by multiplying the Average with `1` and adding `0`."],["BlockHashCount",""],["DeletionQueueDepth",""],["DeletionWeightLimit",""],["DepositPerByte",""],["DepositPerItem",""],["ExistentialDeposit",""],["ExtrinsicBaseWeight","Time to execute a NO-OP extrinsic, for example `System::remark`. Calculated by multiplying the Average with `1` and adding `0`."],["GenesisConfig",""],["IdentityFee","Implementor of `WeightToFee` that maps one unit of weight to one unit of fee."],["IndexDeposit",""],["MaxAdditionalFields",""],["MaxAuthorities",""],["MaxCodeSize",""],["MaxLocks",""],["MaxRegistrars",""],["MaxValueSize",""],["MetadataDepositBase",""],["MetadataDepositPerByte",""],["MinimumPeriod",""],["MySchedule",""],["OperationalFeeMultiplier",""],["Origin","The runtime origin type representing the origin of a call."],["PalletInfo","Provides an implementation of `PalletInfo` to provide information about the pallet setup in the runtime."],["Perbill","A fixed point representation of a number in the range [0, 1]."],["Permill","A fixed point representation of a number in the range [0, 1]."],["RocksDbWeight","By default, Substrate uses RocksDB, so this will be the weight used throughout the runtime."],["Runtime",""],["RuntimeApi",""],["RuntimeApiImpl","Implements all runtime apis for the client side."],["RuntimeBlockLength","We allow for 2 seconds of compute with a 6 second average block time."],["RuntimeBlockWeights",""],["SS58Prefix",""],["Schedule","Definition of the cost schedule and other parameterizations for the wasm vm."],["StorageBytesMultiplier",""],["StorageInfo","Metadata about storage from the runtime."],["StringLimit",""],["TicketsAssetId",""],["Version",""]],"trait":[["BuildStorage","Complex storage builder stuff."],["Contains","A trait for querying whether a type can be said to “contain” a value."],["KeyOwnerProofSystem","Something which can compute and check proofs of a historical key owner and return full identification data of that key owner."],["Randomness","A trait that is able to provide randomness."],["StorageValue","A trait for working with macro-generated storage values under the substrate storage API."]],"type":[["AccountId","Some way of identifying an account on the chain. We intentionally make it equivalent to the public key of our transaction signing scheme."],["AccountIndex","Related to Index pallet"],["Accounts",""],["AccountsConfig",""],["Address","The address format for describing accounts."],["AllPallets","All pallets included in the runtime as a nested tuple of types."],["AllPalletsReversedWithSystemFirst","All pallets included in the runtime as a nested tuple of types in reversed order. With the system pallet first."],["AllPalletsWithSystem","All pallets included in the runtime as a nested tuple of types."],["AllPalletsWithSystemReversed","All pallets included in the runtime as a nested tuple of types in reversed order."],["AllPalletsWithoutSystem","All pallets included in the runtime as a nested tuple of types. Excludes the System pallet."],["AllPalletsWithoutSystemReversed","All pallets included in the runtime as a nested tuple of types in reversed order. Excludes the System pallet."],["Assets",""],["AssetsConfig",""],["Aura",""],["AuraConfig",""],["Balance","Balance of an account."],["Balances",""],["BalancesConfig",""],["Block","Block type as expected by this runtime."],["BlockNumber","An index to a block."],["Contracts",""],["Detach",""],["DetachConfig",""],["Executive","Executive: handles dispatch to the various modules."],["Fragments",""],["Grandpa",""],["GrandpaConfig",""],["Hash","A hash of some data used by the chain."],["Header","Block header type as expected by this runtime."],["Identity",""],["Index","Index of a transaction in the chain."],["Indices",""],["IndicesConfig",""],["Multisig",""],["Protos",""],["Proxy",""],["RandomnessCollectiveFlip",""],["Signature","Alias to 512-bit hash when used in the context of a transaction signature on the chain."],["SignedExtra","The SignedExtension to the basic transaction logic."],["SignedPayload","The payload being signed in transactions."],["Sudo",""],["SudoConfig",""],["System",""],["SystemConfig",""],["Timestamp",""],["TransactionPayment",""],["TransactionPaymentConfig",""],["UncheckedExtrinsic","Unchecked extrinsic type as expected by this runtime."],["Utility",""],["Weight","Numeric range of a transaction weight."]]};