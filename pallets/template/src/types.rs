use codec::{Decode, Encode};
use scale_info::TypeInfo;

//DIDData types
#[derive(PartialEq, Eq, TypeInfo, Clone, Encode, Decode, Debug)]
pub struct DIDData{
    pub id: u64,
    pub data: Vec<u8>,
}

