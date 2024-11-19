use sha3::{Digest, Sha3_256};

type MerkleTree = Vec<Vec<[u8; 32]>>;

pub fn create_merkle_tree(inputs: Vec<&str>) {
    let mut tree: MerkleTree = Vec::new();
    let needs_level = inputs.len() as f32 / 2. + 1.;
    let leaves: Vec<&[u8]> = inputs.iter().map(|x| x.as_bytes()).collect();

    create_first_leaves(&mut tree, leaves);

    while tree.len() < needs_level as usize {
        let actual_level = tree.len();
        create_next_leaves(&mut tree, actual_level);
    }

    for (i, level) in tree.iter().enumerate() {
        println!("Layer: {}", i);
        for hash in level {
            println!("{}", hex::encode(hash));
        }
    }
}

fn create_first_leaves(tree: &mut MerkleTree, leaves: Vec<&[u8]>) {
    let mut first_leaves: Vec<[u8; 32]> = Vec::new();

    for leaf in leaves {
        let hashed_value = hash(leaf);
        first_leaves.push(hashed_value);
    }

    tree.push(first_leaves);
}

fn create_next_leaves(tree: &mut MerkleTree, level: usize) {
    let mut next_leaves: Vec<[u8; 32]> = Vec::new();

    // I need [len(previous leaves) / 2] nodes
    for i in 0..(tree[level - 1].len() / 2) {
        let l_hash = &tree[level - 1][i * 2];
        let r_hash = &tree[level - 1][i * 2 + 1];

        let mut hasher = Sha3_256::new();
        hasher.update(l_hash);
        hasher.update(r_hash);
        let hashed_values = hasher.finalize();
        next_leaves.push(hashed_values.into());
    }

    tree.push(next_leaves);
}

fn hash(value: &[u8]) -> [u8; 32] {
    let mut hasher = Sha3_256::new();
    hasher.update(value);
    let hashed_value = hasher.finalize_reset();
    hashed_value.into()
}
