use sha3::{Digest, Sha3_256};

fn main() {
    let enconde = hex::encode(Sha3_256::digest(b"abc"));
    assert_eq!(
        enconde,
        "3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532"
    )
}
