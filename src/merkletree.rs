use crypto::digest::Digest;

use tree::{ Tree };
use hashable::{ Hashable };
use merkledigest::{ MerkleDigest };

pub struct MerkleTree<D, T> {
    digest: D,
    tree: Tree<T>,
    pub height: usize,
    pub count: usize
}

impl <D, T> MerkleTree<D, T> where D: Digest, T: Hashable {

    pub fn from_vec(mut digest: D, values: Vec<T>) -> Self {
        if values.is_empty() {
            panic!("Cannot build a Merkle tree from an empty vector.");
        }

        let count  = values.len();
        let mut height = 0;
        if count == 1 {
            height = 1;
        }

        let mut cur = Vec::with_capacity(count);

        for v in values.into_iter().rev() {
            let leaf = Tree::make_leaf(&mut digest, v);
            cur.push(leaf);
        }

        cur.reverse();

        while cur.len() > 1 {
            let mut next = Vec::new();
            while cur.len() > 0 {
                if cur.len() == 1 {
                    next.push(cur.remove(0));
                }
                else {
                    let left  = cur.remove(0);
                    let right = cur.remove(0);

                    let combined_hash = digest.combine_hashes(
                        left.get_hash(),
                        right.get_hash()
                    );

                    let node = Tree::Node {
                       hash: combined_hash,
                       left: Box::new(left),
                       right: Box::new(right)
                    };

                    next.push(node);
                }
            }
            height = height + 1;
            cur = next;
        }

        assert!(cur.len() == 1);

        let tree = cur.remove(0);

        MerkleTree {
            digest: digest,
            tree: tree,
            height: height,
            count: count
        }
    }

    pub fn root_hash(&self) -> &Vec<u8> {
        self.tree.get_hash()
    }

}