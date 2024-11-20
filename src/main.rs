use tree::{add_element, create_merkle_tree, create_proof, print_tree};

mod tree;

fn main() {
    let mut tree = create_merkle_tree(vec![b"1", b"2", b"3", b"4"]);

    print_tree(&tree);

    let proof = create_proof(&tree, b"1").unwrap();
    for a in proof.iter() {
        println!("{:#?}", hex::encode(a));
    }
}
