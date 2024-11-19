use tree::create_merkle_tree;

mod tree;

fn main() {
    let tree = create_merkle_tree(vec![b"1", b"2", b"3", b"4"]);

    for (i, level) in tree.iter().enumerate() {
        println!("Layer: {}", i);
        for hash in level {
            println!("{}", hex::encode(hash));
        }
    }
}
