window.SIDEBAR_ITEMS = {"enum":[["Call","Contains one variant per dispatchable that can be called by an extrinsic."],["Error","Custom dispatch errors of this pallet."],["Event","The event emitted by this pallet."]],"struct":[["Pallet","The pallet implementing the on-chain logic."]],"trait":[["Config","Configure the pallet by specifying the parameters and types on which it depends."]],"type":[["CopiesCount","StorageMap that maps a tuple that contains a Fragment Definition ID and an Edition ID to the total number of Fragment Instances that have the Fragment Definition ID and the Edition ID"],["Definitions","StorageMap that maps a Fragment Definition ID (which is determinstically computed using its Proto-Fragment hash and its metadata struct `FragmentMetadata`) to a FragmentDefinition struct"],["EditionsCount","StorageMap that maps a Fragment Definition ID to the total number of unique Edition IDs found in the Fragment Instances that have the aforementioned Fragment Definition ID"],["Expirations","StorageMap that maps the Block Number that a Fragment Instance expires at to a tuple that contains the Fragment Instance’s Fragment Definition ID, the Fragment Instance’s Edition ID and the Fragment Instance’s Copy ID"],["Fragments","StorageNMap that maps the Fragment Definition ID of a Fragment Instance, the Fragment Edition ID of the aforementioned Fragment Instance and the Copy ID of the aforementioned Fragment Instance to a `FragmentInstance` struct"],["Inventory","StorageDoubleMap that maps the Owner of a Fragment Instance and the Fragment Instance’s Fragment Definition to a tuple that contains the Fragment Instance’s Edition ID and the Fragment Instance’s Copy ID"],["Module","Type alias to `Pallet`, to be used by `construct_runtime`."],["Owners","StorageDoubleMap that maps a Fragment Definition and the Owner of a Fragment Instance that was created from the aforementioned Fragment Definition to a tuple that contains the Fragment Instance’s Edition ID and the Fragment Instance’s Copy ID"],["Proto2Fragments","StorageMap that maps a Proto-Fragment to a list of Fragment Definitions that were created using the aforementioned Proto-Fragment"],["Publishing","StorageMap that maps a Fragment Definition ID (which is determinstically computed using its Proto-Fragment hash and its metadata struct `FragmentMetadata`) to a PublishingData struct (of the aforementioned Fragment Definition)"],["UniqueData2Edition",""]]};