use tree::{add_element, create_merkle_tree, create_proof, print_tree, verify_proof};

mod tree;

fn main() {
    let mut tree = create_merkle_tree(vec![b"1", b"2", b"3"]);
    tree = add_element(&mut tree, b"4");
    print_tree(&tree);

    let proof = create_proof(&tree, b"4").unwrap();
    let verify = verify_proof(&tree, &proof, b"4");

    println!("{}", verify);
}
