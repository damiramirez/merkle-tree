use sha3::{Digest, Sha3_256};

type MerkleTree = Vec<Vec<[u8; 32]>>;

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
    let mut first_leaves: Vec<[u8; 32]> = Vec::new();

    for leaf in &leaves {
        let hash_value = hash_one(leaf);
        first_leaves.push(hash_value);
    }

    tree.push(first_leaves);
}

fn create_next_leaves(tree: &mut MerkleTree, level: usize) {
    let mut next_leaves: Vec<[u8; 32]> = Vec::new();
    let mut hasher = Sha3_256::new();

    for i in 0..tree[level - 1].len().div_ceil(2) {
        let l_hash: &[u8; 32] = &tree[level - 1][i * 2];
        hasher.update(l_hash);

        let r_hash = &tree[level - 1].get(i * 2 + 1);
        // Check if right node exists, if not use the left node
        match r_hash {
            Some(hash) => hasher.update(hash),
            None => hasher.update(l_hash),
        }

        let hash_values = hasher.finalize_reset();
        next_leaves.push(hash_values.into());
    }

    tree.push(next_leaves);
}

fn hash_one(value: &[u8]) -> [u8; 32] {
    let mut hasher = Sha3_256::new();
    hasher.update(value);
    let hashed_value = hasher.finalize_reset();
    hashed_value.into()
}

#[cfg(test)]
fn get_root(tree: &MerkleTree) -> Option<[u8; 32]> {
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
}
