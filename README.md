# merkle-tree

Implementation of [Merkle Tree](https://en.wikipedia.org/wiki/Merkle_tree) in Rust.

## Usage

```bash
git clone git@github.com:damiramirez/merkle-tree.git && cd merkle-tree
make run
```

## Functions

- Create a new Merkle Tree with a vector of bytes
- Add elements to the tree
- Get the root hash of the Merkle Tree
- Get the proof of existence of a leaf node and verify it

### Example

```rust
use tree::{add_element, create_merkle_tree, create_proof, print_tree, verify_proof};

mod tree;

fn main() {
    let empty: &[String] = &[];
    let mut tree = create_merkle_tree(empty);
    tree = add_element(&mut tree, b"4");
    print_tree(&tree);

    if let Some(proof) = create_proof(&tree, b"4") {
        assert!(verify_proof(&tree, &proof, b"4"));
    }
}
```
