window.SIDEBAR_ITEMS = {"enum":[["Call","Contains one variant per dispatchable that can be called by an extrinsic."],["Error","Custom dispatch errors of this pallet."],["Event","The event emitted by this pallet."]],"struct":[["Pallet","The pallet implementing the on-chain logic."]],"trait":[["Config","Configure the pallet by specifying the parameters and types on which it depends."]],"type":[["AccountCurations","StorageMap that maps a Clamor Account ID to a list of Proto-Fragments that was staked on by the aforementioned Clamor Account ID"],["ExpiringCurations","StorageMap that maps a Block number to a list of accounts that have curations expiring on that block number"],["MetaKeys","StorageMap that maps a Metadata Key (of type `Vec<u8>`) to an index number"],["MetaKeysIndex","StorageValue that equals the total number of unique Metadata Keys in the blockchain"],["Module","Type alias to `Pallet`, to be used by `construct_runtime`."],["ProtoCurations","StorageDoubleMap that maps a Proto-Fragment and a Clamor Account ID to a tuple that contains the Curated Amount (tickets burned by the aforementioned Clamor Account ID) and the Block Number"],["Protos","StorageMap that maps a Proto-Fragment’s data’s hash to a Proto struct (of the aforementioned Proto-Fragment)"],["ProtosByCategory","StorageMap that maps a variant of the Category enum to a list of Proto-Fragment hashes (that have the aforementioned variant)"],["ProtosByOwner","StorageMap that maps a variant of the ProtoOwner enum to a list of Proto-Fragment hashes (that have the aforementioned variant)"],["Tags","StorageMap that maps a Tag (of type `Vec<u8>`) to an index number"],["TagsIndex","StorageValue that equals the total number of unique tags in the blockchain"],["Traits","StorageMap that maps a Trait ID to the name of the Trait itself"]]};