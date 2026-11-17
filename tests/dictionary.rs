use project_custom_avl_dictionary::tree::Node;
use project_custom_avl_dictionary::{Dictionary, dictionary};
use std::collections::BTreeMap;

/// Checks the BST order and the AVL balance of the subtree and returns its height.
fn check_avl(node: *const Node, low: Option<u64>, high: Option<u64>) -> i32 {
    if node.is_null() {
        return 0;
    }
    let node = unsafe { &*node };
    if let Some(low) = low {
        assert!(node.key > low, "BST order broken at key {}", node.key);
    }
    if let Some(high) = high {
        assert!(node.key < high, "BST order broken at key {}", node.key);
    }
    let left = check_avl(node.left, low, Some(node.key));
    let right = check_avl(node.right, Some(node.key), high);
    assert!(
        (left - right).abs() <= 1,
        "node {} is unbalanced: left height {left}, right height {right}",
        node.key
    );
    1 + left.max(right)
}

/// A small deterministic pseudo-random generator, so the test needs no extra crates.
struct Lcg(u64);

impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.0 >> 33
    }
}

#[test]
fn random_operations_match_btreemap_and_keep_the_tree_balanced() {
    let operations = if cfg!(miri) { 300 } else { 5000 };
    let mut rng = Lcg(42);
    let mut dict = Dictionary::new();
    let mut reference = BTreeMap::new();

    for i in 0..operations {
        let key = rng.next() % 200;
        if rng.next() % 3 == 0 {
            assert_eq!(dict.remove(key), reference.remove(&key).is_some());
        } else {
            let value = format!("value-{i}");
            dict.insert(key, &value);
            reference.insert(key, value);
        }

        assert_eq!(dict.len(), reference.len());
        if i % 50 == 0 {
            check_avl(dict.root(), None, None);
        }
    }

    check_avl(dict.root(), None, None);
    for key in 0..200 {
        assert_eq!(dict.get(key), reference.get(&key).map(String::as_str));
        assert_eq!(dict.contains(key), reference.contains_key(&key));
    }
}

#[test]
fn sorted_inserts_keep_logarithmic_height() {
    let count: u64 = if cfg!(miri) { 255 } else { 100_000 };
    let mut dict = Dictionary::new();
    for key in 0..count {
        dict.insert(key, "x");
    }
    let height = check_avl(dict.root(), None, None);
    let bound = 1.45 * ((count + 2) as f64).log2();
    assert!(
        (height as f64) <= bound,
        "height {height} exceeds the AVL bound {bound:.1}"
    );
}

#[test]
fn insert_replaces_value_and_remove_reports_presence() {
    let mut dict = dictionary! { 1 => "a", 2 => "b", 3 => "c" };
    assert_eq!(dict.len(), 3);

    dict.insert(2, "bb");
    assert_eq!(dict.get(2), Some("bb"));
    assert_eq!(dict.len(), 3);

    assert!(dict.remove(2));
    assert!(!dict.remove(2));
    assert_eq!(dict.get(2), None);
    assert_eq!(dict.len(), 2);
    assert!(!dict.is_empty());
}

#[test]
fn empty_dictionary() {
    let mut dict = Dictionary::default();
    assert!(dict.is_empty());
    assert_eq!(dict.get(7), None);
    assert!(!dict.remove(7));
}
