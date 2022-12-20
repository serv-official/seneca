use codec::{Decode, Encode};
use scale_info::TypeInfo;

//DIDData types
#[derive(PartialEq, Eq, PartialOrd, Ord, TypeInfo, Clone, Encode, Decode, Debug)]
pub struct DIDData{
    pub credential_subject: CredentialSubject,
    pub issuer: Issuer,
    pub id: u64,
    pub credential_type: CredentialType,
    pub context: Context,
    pub issuing_date: String,
    pub proof: IdProof,

}

#[derive(PartialEq, Eq, PartialOrd, Ord, TypeInfo, Clone, Encode, Decode, Debug)]
pub struct CredentialSubject{
    pub name: String,
    pub id: u64,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, TypeInfo, Clone, Encode, Decode, Debug)]
pub struct Issuer{
    pub id: u64,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, TypeInfo, Clone, Encode, Decode, Debug)]
pub struct CredentialType{
    pub name: String,
    pub cred_type: String,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, TypeInfo, Clone, Encode, Decode, Debug)]
pub struct Context{
    pub name: String,
    pub context: String,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, TypeInfo, Clone, Encode, Decode, Debug)]
pub struct IdProof{
    pub proof_type: String,
    pub proof_value: String,
}