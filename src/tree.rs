use sha3::{Digest, Sha3_256};

pub type Hash = [u8; 32];

type MerkleTree = Vec<Vec<Hash>>;

pub fn create_merkle_tree(leaves: Vec<&[u8]>) -> MerkleTree {
    let mut tree: MerkleTree = Vec::new();

    create_first_leaves(&mut tree, leaves);
    // Formula to calculate the levels
    let needs_level = (tree[0].len() as f32).log2().ceil() + 1.;

    while tree.len() < needs_level as usize {
        let actual_level = tree.len();
        create_next_leaves(&mut tree, actual_level);
    }

    tree
}

fn create_first_leaves(tree: &mut MerkleTree, leaves: Vec<&[u8]>) {
    let mut first_leaves: Vec<Hash> = Vec::new();

    for leaf in &leaves {
        let hash_value = hash_one(leaf);
        first_leaves.push(hash_value);
    }

    tree.push(first_leaves);
}

fn create_next_leaves(tree: &mut MerkleTree, level: usize) {
    let mut next_leaves: Vec<Hash> = Vec::new();

    for i in 0..tree[level - 1].len().div_ceil(2) {
        let l_hash: &Hash = &tree[level - 1][i * 2];
        let r_hash = match &tree[level - 1].get(i * 2 + 1) {
            Some(hash) => hash,
            None => l_hash,
        };

        next_leaves.push(hash_multiple(&[l_hash, r_hash]));
    }

    tree.push(next_leaves);
}

fn hash_one(value: &[u8]) -> Hash {
    let mut hasher = Sha3_256::new();
    hasher.update(value);
    let hashed_value = hasher.finalize_reset();
    hashed_value.into()
}

fn hash_multiple(values: &[&[u8]]) -> Hash {
    let mut hasher = Sha3_256::new();
    for value in values {
        hasher.update(value);
    }
    let hashed_value = hasher.finalize();
    hashed_value.into()
}

pub fn add_element(tree: &mut MerkleTree, value: &[u8]) -> MerkleTree {
    // Re-create the merkle tree with the new value
    let mut new_tree: MerkleTree = Vec::new();

    let mut first_leaves = tree.first().cloned().unwrap_or_default();
    first_leaves.push(hash_one(value));

    new_tree.push(first_leaves);
    let needs_level = (new_tree[0].len() as f32).log2().ceil() + 1.;

    while new_tree.len() < needs_level as usize {
        let actual_level = new_tree.len();
        create_next_leaves(&mut new_tree, actual_level);
    }

    new_tree
}

pub fn print_tree(tree: &MerkleTree) {
    for (i, level) in tree.iter().enumerate() {
        println!("Layer: {}", i);
        for hash in level {
            println!("\t{}", hex::encode(hash));
        }
    }
}

pub fn create_proof(tree: &MerkleTree, value: &[u8]) -> Option<(Vec<Hash>, usize)> {
    let mut proof = Vec::new();
    let leaves = tree.first()?;
    let mut index = get_leaf(leaves, &hash_one(value))?;

    // Avoid root
    let level = tree.len() - 1;

    for layer in tree[0..level].iter() {
        if index % 2 == 0 {
            // Check if right node is missing
            let sibling = match layer.get(index + 1) {
                Some(r_hash) => r_hash,
                None => &leaves[index],
            };
            proof.push(*sibling);
        } else {
            proof.push(*layer.get(index - 1).unwrap());
        }

        index /= 2;
    }

    Some((proof, index))
}

pub fn verify_proof(tree: &MerkleTree, proof: Vec<Hash>, value: &[u8]) -> bool {
    let root = get_root(tree).unwrap();
    let mut hash_value = hash_one(value);
    let leaves = tree.first().unwrap();
    let mut index = get_leaf(leaves, &hash_one(value)).unwrap();

    for pp in proof.iter() {
        if index % 2 == 0 {
            hash_value = hash_multiple(&[&hash_value, pp])
        } else {
            hash_value = hash_multiple(&[pp, &hash_value])
        }

        index /= 2;
    }

    root == hash_value
}

fn get_leaf(layer: &[Hash], value: &Hash) -> Option<usize> {
    layer.iter().position(|elem| elem == value)
}

fn get_root(tree: &MerkleTree) -> Option<Hash> {
    tree.last().and_then(|level| level.first().cloned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_tree() {
        let tree = create_merkle_tree(Vec::new());
        assert_eq!(get_root(&tree), None);
    }

    #[test]
    fn tree_power_of_2_with_4_leaves() {
        let tree = create_merkle_tree(vec![b"1", b"2", b"3", b"4"]);
        assert_eq!(
            get_root(&tree),
            Some([
                137, 153, 44, 123, 164, 130, 79, 195, 21, 135, 186, 74, 94, 220, 125, 98, 73, 20,
                100, 119, 87, 220, 77, 185, 218, 60, 243, 252, 72, 120, 28, 89
            ]),
        );
    }

    #[test]
    fn tree_not_power_of_2_with_6_leaves() {
        let tree = create_merkle_tree(vec![b"1", b"2", b"3", b"4", b"5", b"6"]);
        assert_eq!(
            get_root(&tree),
            Some([
                150, 175, 62, 140, 117, 101, 10, 4, 24, 24, 124, 179, 100, 93, 142, 72, 141, 188,
                224, 58, 237, 118, 71, 58, 207, 196, 14, 41, 47, 173, 190, 67,
            ]),
        );
    }

    #[test]
    fn tree_add_element() {
        let mut tree = create_merkle_tree(vec![b"1", b"2", b"3"]);
        tree = add_element(&mut tree, b"4");
        assert_eq!(
            get_root(&tree),
            Some([
                137, 153, 44, 123, 164, 130, 79, 195, 21, 135, 186, 74, 94, 220, 125, 98, 73, 20,
                100, 119, 87, 220, 77, 185, 218, 60, 243, 252, 72, 120, 28, 89
            ]),
        );
    }
}
