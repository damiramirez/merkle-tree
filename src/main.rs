use tree::{add_element, create_merkle_tree};

mod tree;

fn main() {
    let mut tree = create_merkle_tree(vec![b"1", b"2", b"3"]);

    for (i, level) in tree.iter().enumerate() {
        println!("Layer: {}", i);
        for hash in level {
            println!("{}", hex::encode(hash));
        }
    }

    tree = add_element(&mut tree, b"4");

    for (i, level) in tree.iter().enumerate() {
        println!("Layer: {}", i);
        for hash in level {
            println!("{}", hex::encode(hash));
        }
    }

    tree = add_element(&mut tree, b"5");

    for (i, level) in tree.iter().enumerate() {
        println!("Layer: {}", i);
        for hash in level {
            println!("{}", hex::encode(hash));
        }
    }
}
