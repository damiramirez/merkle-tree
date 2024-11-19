use tree::create_merkle_tree;

mod tree;

fn main() {
    create_merkle_tree(vec!["1", "2", "3", "4"]);
}
