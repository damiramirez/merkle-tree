use sha3::{Digest, Sha3_256};

type MerkleTree = Vec<Vec<[u8; 32]>>;

pub fn create_merkle_tree(inputs: Vec<&str>) {
    let mut tree: MerkleTree = Vec::new();
    let leaves: Vec<&[u8]> = inputs.iter().map(|x| x.as_bytes()).collect();

    create_first_leaves(&mut tree, leaves);

    for leaf in tree {
        println!("{:#?}", leaf);
    }
}

fn create_first_leaves(tree: &mut MerkleTree, leaves: Vec<&[u8]>) {
    let mut first_layer: Vec<[u8; 32]> = Vec::new();
    let mut hasher = Sha3_256::new();

    for leaf in leaves {
        hasher.update(leaf);
        let hashed_value = hasher.finalize_reset();
        let hashed_array: [u8; 32] = hashed_value.into();
        first_layer.push(hashed_array);
    }

    tree.push(first_layer);
}
