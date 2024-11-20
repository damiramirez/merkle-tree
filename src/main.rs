use tree::{add_element, create_merkle_tree, print_tree};

mod tree;

fn main() {
    let mut tree = create_merkle_tree(vec![b"1"]);
    print_tree(&tree);

    tree = add_element(&mut tree, b"2");
    print_tree(&tree);

    tree = add_element(&mut tree, b"3");
    tree = add_element(&mut tree, b"4");

    print_tree(&tree);
}
