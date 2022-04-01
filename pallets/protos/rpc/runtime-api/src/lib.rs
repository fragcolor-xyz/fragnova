#![cfg_attr(not(feature = "std"), no_std)]
use codec::Codec;
use sp_clamor::Hash256;
use sp_std::vec::Vec;

sp_api::decl_runtime_apis! {
		pub trait ProtosApi<Tags> where
				Tags: Codec, {
				fn get_by_tag(tags: Tags) -> Option<Vec<Hash256>>;
		}
}