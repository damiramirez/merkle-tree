use tree::{add_element, create_merkle_tree, create_proof, print_tree, verify_proof};

mod tree;

fn main() {
    let empty: &[String] = &[];
    let mut tree = create_merkle_tree(empty);
    tree = add_element(&mut tree, b"1");
    tree = add_element(&mut tree, b"2");
    tree = add_element(&mut tree, b"3");
    tree = add_element(&mut tree, b"4");
    print_tree(&tree);

    let proof = create_proof(&tree, b"2").unwrap();
    let verify = verify_proof(&tree, &proof, b"2");

    println!("{}", verify);
}
