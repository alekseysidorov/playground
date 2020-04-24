use std::{borrow::Borrow, cmp::Ordering, mem};

const T: usize = 2;
const MAX_LEN: usize = 2 * T - 1;
const MID: usize = T - 1;
const RIGHT: usize = MID + 1;

#[derive(Debug)]
struct KeyValue<K, V> {
    key: K,
    value: V,
}

impl<K: Ord, V> PartialEq for KeyValue<K, V> {
    fn eq(&self, other: &KeyValue<K, V>) -> bool {
        self.key.eq(&other.key)
    }
}

impl<K: Ord, V> Eq for KeyValue<K, V> {}

impl<K: Ord, V> PartialOrd for KeyValue<K, V> {
    fn partial_cmp(&self, other: &KeyValue<K, V>) -> Option<Ordering> {
        self.key.partial_cmp(&other.key)
    }
}

impl<K: Ord, V> Ord for KeyValue<K, V> {
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
        let kv = KeyValue { key, value };

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
        self.root.find_key(key)
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
}

#[derive(Debug, PartialEq)]
enum BalanceState {
    Finished,
    UnbalancedLeaf,
}

#[derive(Default, Debug)]
struct Node<K: Ord, V> {
    values: Vec<KeyValue<K, V>>,
    children: Vec<Node<K, V>>,
}

impl<K: Ord, V> Node<K, V> {
    fn new_leaf(kv: KeyValue<K, V>) -> Self {
        Self {
            values: vec![kv],
            children: vec![],
        }
    }

    fn insert(&mut self, kv: KeyValue<K, V>) -> Option<(KeyValue<K, V>, Node<K, V>)> {
        if !self.is_leaf() {
            let child_index = match self.values.binary_search(&kv) {
                // Just modify value.
                Ok(idx) => {
                    self.values[idx].value = kv.value;
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

    fn find_key<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        match self
            .values
            .binary_search_by(|probe| key.cmp(probe.key.borrow()).reverse())
        {
            Ok(idx) => Some(&self.values[idx].value),

            Err(child_index) => {
                if child_index < self.children.len() {
                    self.children[child_index].find_key(key)
                } else {
                    None
                }
            }
        }
    }

    fn insert_leaf(&mut self, kv: KeyValue<K, V>) -> Option<(KeyValue<K, V>, Node<K, V>)> {
        self.insert_kv(kv);

        if self.need_split() {
            let right_values = self.values.drain(RIGHT..).collect();
            let right_children = if self.children.len() > RIGHT {
                self.children.drain(RIGHT..).collect()
            } else {
                vec![]
            };
            let mid = self.values.pop().unwrap();

            Some((
                mid,
                Node {
                    values: right_values,
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
            .values
            .binary_search_by(|probe| key.cmp(probe.key.borrow()).reverse())
        {
            Ok(idx) => {
                if self.is_leaf() {
                    self.remove_leaf(idx)
                } else {
                    todo!()
                }
            }

            Err(child_index) => {
                if child_index < self.children.len() {
                    dbg!(&self, child_index);

                    let (value, state) = self.children[child_index].remove(key);

                    match state {
                        BalanceState::UnbalancedLeaf => {

                        },
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

    fn remove_leaf(&mut self, idx: usize) -> (Option<V>, BalanceState) {
        self.values[idx..].rotate_left(1);

        match self.values.pop() {
            None => (None, BalanceState::Finished),
            Some(x) if self.values.len() > MID => (Some(x.value), BalanceState::Finished),
            Some(x) => (Some(x.value), BalanceState::UnbalancedLeaf),
        }
    }

    fn insert_kv(&mut self, kv: KeyValue<K, V>) {
        let idx = match self.values.binary_search(&kv) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };

        self.values.push(kv);
        if idx < self.values.len() {
            // Rotate tail right.
            self.values[idx..].rotate_right(1);
        }
    }

    fn need_split(&self) -> bool {
        self.values.len() == MAX_LEN
    }

    // Indicates whether node is terminal or not.
    fn is_leaf(&self) -> bool {
        self.children.is_empty()
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

    map.remove(&10);

    dbg!(&map);
}
