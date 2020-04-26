use std::{borrow::Borrow, cmp::Ordering, mem};

const TREE_ORDER: usize = 2;
const MAX_LEN: usize = 2 * TREE_ORDER - 1;
const MID: usize = TREE_ORDER - 1;
const RIGHT: usize = MID + 1;

#[derive(Debug)]
struct Item<K, V> {
    key: K,
    value: V,
}

impl<K: Ord, V> PartialEq for Item<K, V> {
    fn eq(&self, other: &Item<K, V>) -> bool {
        self.key.eq(&other.key)
    }
}

impl<K: Ord, V> Eq for Item<K, V> {}

impl<K: Ord, V> PartialOrd for Item<K, V> {
    fn partial_cmp(&self, other: &Item<K, V>) -> Option<Ordering> {
        self.key.partial_cmp(&other.key)
    }
}

impl<K: Ord, V> Ord for Item<K, V> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}

#[derive(Default, Debug)]
pub struct NaiveBTree<K: Ord, V> {
    root: Node<K, V>,
}

impl<K: Ord, V> NaiveBTree<K, V> {
    pub fn insert(&mut self, key: K, value: V) {
        let kv = Item { key, value };

        if let Some((kv, right)) = self.root.insert(kv) {
            let mut new_root = Node::new_leaf(kv);
            mem::swap(&mut new_root, &mut self.root);
            self.root.children = vec![new_root, right];
        }
    }

    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        self.root.get(key)
    }

    pub fn remove<Q: ?Sized>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Ord,
        K: std::fmt::Debug,
        V: std::fmt::Debug,
    {
        let (value, state) = self.root.remove(key);
        value
    }

    pub fn insert_2(&mut self, key: K, value: V) {
        // Grow upwards if the root is full.
        if self.root.is_full() {
            let mut new_root = Node::new_leaf(Item { key, value });
            mem::swap(&mut new_root, &mut self.root);
            self.root.children = vec![new_root];
            self.root.split_child(0);
        }

        // Work down the tree
        let mut current = &mut self.root;
        while !current.is_leaf() {

        }


        let kv = Item { key, value };

        if let Some((kv, right)) = self.root.insert(kv) {
            let mut new_root = Node::new_leaf(kv);
            mem::swap(&mut new_root, &mut self.root);
            self.root.children = vec![new_root, right];
        }
    }


    pub fn remove_2<Q: ?Sized>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Ord,
        K: std::fmt::Debug,
        V: std::fmt::Debug,
    {
        let mut current = &mut self.root;

        loop {
            match current.find_key(key) {
                // If the item to remove has been found.
                Ok(index) => {
                    dbg!(&current, &index);

                    let result;
                    if current.is_leaf() {
                        // If current node is leaf, just remove item.
                        // FIXME: Check if we should modify tree on the more upper level.
                        result = current.remove_item_2(index);
                    } else {
                        // Otherwise replace with predecessor or successor or merge children.
                        let mut left = &current.children[index];
                        if left.keys.len() >= TREE_ORDER {}

                        todo!()
                    }

                    return result.map(|x| x.value);
                }

                // If the node has not been found go down the tree.
                Err(child_index) => {
                    // A leaf was reached, and the desired key was not found.
                    if child_index >= current.children.len() {
                        return None;
                    }

                    // Go down the tree and adjust current node.
                    current = &mut current.children[child_index];
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum BalanceState {
    Finished,
    UnbalancedLeaf,
}

#[derive(Debug, PartialEq)]
enum FixAction {
    Unmodified,
    ModifiedNotRoot,
    NewRoot,
}

#[derive(Default, Debug)]
struct Node<K: Ord, V> {
    keys: Vec<Item<K, V>>,
    children: Vec<Node<K, V>>,
}

impl<K: Ord, V> Node<K, V> {
    fn new_leaf(kv: Item<K, V>) -> Self {
        Self {
            keys: vec![kv],
            children: vec![],
        }
    }

    fn insert(&mut self, kv: Item<K, V>) -> Option<(Item<K, V>, Node<K, V>)> {
        if !self.is_leaf() {
            let child_index = match self.keys.binary_search(&kv) {
                // Just modify value.
                Ok(idx) => {
                    self.keys[idx].value = kv.value;
                    return None;
                }

                Err(idx) => idx,
            };

            if let Some((kv, right)) = self.children[child_index].insert(kv) {
                self.children.push(right);
                self.insert_leaf(kv)
            } else {
                None
            }
        } else {
            self.insert_leaf(kv)
        }
    }

    fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        match self.find_key(key) {
            Ok(idx) => Some(&self.keys[idx].value),

            Err(child_index) => {
                if child_index < self.children.len() {
                    self.children[child_index].get(key)
                } else {
                    None
                }
            }
        }
    }

    fn insert_leaf(&mut self, kv: Item<K, V>) -> Option<(Item<K, V>, Node<K, V>)> {
        self.insert_item(kv);

        if self.is_full() {
            let right_values = self.keys.drain(RIGHT..).collect();
            let right_children = if self.children.len() > RIGHT {
                self.children.drain(RIGHT..).collect()
            } else {
                vec![]
            };
            let mid = self.keys.pop().unwrap();

            Some((
                mid,
                Node {
                    keys: right_values,
                    children: right_children,
                },
            ))
        } else {
            None
        }
    }

    fn remove<Q: ?Sized>(&mut self, key: &Q) -> (Option<V>, BalanceState)
    where
        K: Borrow<Q>,
        Q: Ord,
        K: std::fmt::Debug,
        V: std::fmt::Debug,
    {
        match self
            .keys
            .binary_search_by(|probe| key.cmp(probe.key.borrow()).reverse())
        {
            Ok(idx) => {
                if self.is_leaf() {
                    self.remove_item(idx)
                } else {
                    todo!()
                }
            }

            Err(child_index) => {
                if child_index < self.children.len() {
                    dbg!(&self, child_index);

                    let (value, state) = self.children[child_index].remove(key);

                    match state {
                        BalanceState::UnbalancedLeaf => {}
                        BalanceState::Finished => {}
                    }

                    dbg!(&state, &value);

                    (value, state)
                } else {
                    (None, BalanceState::Finished)
                }
            }
        }
    }

    fn remove_item(&mut self, idx: usize) -> (Option<V>, BalanceState) {
        self.keys[idx..].rotate_left(1);

        match self.keys.pop() {
            None => (None, BalanceState::Finished),
            Some(x) if self.keys.len() > MID => (Some(x.value), BalanceState::Finished),
            Some(x) => (Some(x.value), BalanceState::UnbalancedLeaf),
        }
    }

    // Parts of a new non-recursive impementation

    // Indicates whether node is terminal or not.
    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    // Indicates whether node has no capacity to add item
    fn is_full(&self) -> bool {
        self.keys.len() >= MAX_LEN
    }

    fn split_child(&mut self, child_index: usize) {
        let child = &mut self.children[child_index];

        let right_values = child.keys.drain(RIGHT..).collect();
        let right_children = if child.children.len() > RIGHT {
            child.children.drain(RIGHT..).collect()
        } else {
            vec![]
        };
        let mid = child.keys.pop().unwrap();

        self.insert_item(mid);
        self.children.push(Node {
            keys: right_values,
            children: right_children,
        });   
    }

    fn insert_item(&mut self, kv: Item<K, V>) -> usize {
        let idx = match self.keys.binary_search(&kv) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };

        self.keys.push(kv);
        if idx < self.keys.len() {
            // Rotate tail right.
            self.keys[idx..].rotate_right(1);
        }

        idx
    }

    fn find_key<Q: ?Sized>(&self, key: &Q) -> Result<usize, usize>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        self.keys
            .binary_search_by(|probe| key.cmp(probe.key.borrow()).reverse())
    }

    fn remove_item_2(&mut self, idx: usize) -> Option<Item<K, V>> {
        self.keys[idx..].rotate_left(1);
        self.keys.pop()
    }

    /// Makes sure that `parent.children[index]` has at least `TREE ORDER` items.
    /// Returns an action that was been taken.
    fn fix_child_size(parent: &mut Node<K, V>, index: usize) -> FixAction {
        let mut child = &mut parent.children[index];

        // If we have to fix child
        if child.keys.len() < TREE_ORDER {
            // Borrow item from the left sibling if possible.
            if index > 0 && parent.children[index - 1].keys.len() >= TREE_ORDER {
                let mut left_child = &mut parent.children[index - 1];
                todo!()
            }
            // Borrow item for the right sibling if possible.
            else if index < parent.children.len()
                && parent.children[index + 1].keys.len() >= TREE_ORDER
            {
                let mut right_child = &mut parent.children[index + 1];

                let mut item = right_child.remove_item_2(0).unwrap();
            }
        }

        FixAction::Unmodified
    }
}

#[test]
fn test_basic() {
    let mut map = NaiveBTree::default();

    let kvs = vec![
        (1, "a"),
        (2, "b"),
        (3, "c"),
        (4, "d"),
        (5, "e"),
        (6, "f"),
        (7, "g"),
        (8, "h"),
        (9, "i"),
        (10, "j"),
    ];

    for (key, value) in &kvs {
        map.insert(*key, *value);
    }

    for (key, value) in &kvs {
        assert_eq!(map.get(key), Some(value));
    }

    assert!(map.get(&11).is_none());

    map.remove_2(&10);

    dbg!(&map);
}
