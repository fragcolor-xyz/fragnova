window.SIDEBAR_ITEMS = {"enum":[["Call","A public part of the pallet."],["Error","Custom dispatch errors of this pallet."],["Event","Events for the pallet."]],"struct":[["GenesisConfig","The Genesis Configuration for the Pallet."],["OraclePrice","Struct used to hold price data received from the Chainlink Price Feed smart contract. Please refer to https://docs.chain.link/docs/data-feeds/price-feeds/api-reference/#latestrounddata."],["Pallet","The pallet implementing the on-chain logic."]],"trait":[["Config","This pallet’s configuration trait"]],"type":[["EVMLinkVoting","StorageMap that maps a FRAG token locking or unlocking event to a number of votes (). The key for this map is: `blake2_256(encoded(<Amount of FRAG token that was locked/unlocked, Signature written by the owner of the FRAG token on a determinstic message, Whether it was locked or unlocked, Ethereum Block Number where it was locked/unlocked>))`"],["EVMLinkVotingClosed","StorageMap that maps a FRAG token locking or unlocking event to a boolean indicating whether voting on the aforementioned event has ended**."],["FragKeys","StorageValue that equals the List of Clamor Account IDs that both validate and send unsigned transactions with signed payload"],["IsOracleStopped","StorageValue that contains the flag used to stop the Oracle."],["Module","Type alias to `Pallet`, to be used by `construct_runtime`."]]};