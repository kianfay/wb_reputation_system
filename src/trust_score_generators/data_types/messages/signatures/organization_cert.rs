use serde::{Deserialize, Serialize};

// This is the certificate granted by an organization's, which includes the organization's public 
// key, the duration(sec) of the certificate and the sig of all this information. This would be 
// derived from a certificate on the participants DID.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OrgCertPreSig {
    pub client_pubkey: String,
    pub duration: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OrgCert {
    pub client_pubkey: String,
    pub duration: u32,
    pub org_pubkey: String,
    pub signature: Vec<u8>
}