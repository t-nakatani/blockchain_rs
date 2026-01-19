use ed25519_dalek::{Signature, SigningKey, VerifyingKey, Signer};
use sha2::{Sha256, digest::Digest};

trait FromPubkey {
    fn from_pubkey(pubkey: &VerifyingKey) -> Self;
}

pub struct PeerId(pub [u8; 32]);

impl FromPubkey for PeerId {
    fn from_pubkey(pubkey: &VerifyingKey) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(pubkey.to_bytes());
        let hash_value = hasher.finalize();

        let mut id = [0; 32];
        id.copy_from_slice(&hash_value);
        Self(id)
    }
}

pub struct Identity {
    sk: SigningKey,
    id: PeerId,
}


impl Identity {
    pub fn new(sk: SigningKey) -> Self {
        let id = PeerId::from_pubkey(&sk.verifying_key());
        Self { sk, id }
    }

    pub fn peer_id(&self) -> &PeerId {
        &self.id
    }

    pub fn sign(&self, message: &[u8]) -> Signature {
        self.sk.sign(message)
    }
}
