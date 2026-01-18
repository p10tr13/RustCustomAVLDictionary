use project_custom_avl_dictionary::{dict, tree};
use project_custom_avl_dictionary::tree::Node;
use std::ptr;

unsafe fn free_dict(root: *mut Node) {
    if root.is_null() {
        return;
    }
    unsafe {
        free_dict((*root).left);
        free_dict((*root).right);
        ptr::drop_in_place(&mut (*root).value);
        libc::free(root as *mut std::ffi::c_void);
    }
}

fn main() {
    unsafe {
        println!("=== AVL LIB (RUST) DEMO ===");

        // 1. Macro
        println!("\n--- 1. Creating dictionary using 'dict!' macro ---");
        let mut root = dict! {
            1 => "a",
            2 => "b",
            3 => "c",
            4 => "d",
            5 => "e"
        };
        println!("State of tree after initialization using macro:");
        tree::print_tree(root, 0);

        let mut h = false;

        // 2. Insert
        println!("\n--- 2. Insert demo ---");
        println!("Adding a new node with key 6 (rotation is expected)");
        tree::insert(&mut root, 6, "f", &mut h);
        println!("Updating a value of a node with key 2 using insert operation");
        tree::insert(&mut root, 2, "bb", &mut h);
        tree::print_tree(root, 0);

        // 3. Find
        println!("\n--- 3. Searching a node using find ---");
        let search_key = 2;
        match tree::find(root, search_key) {
            Some(val_ptr) => {
                println!("Key {}: Val -> '{}'.", search_key, (*val_ptr).as_str());
            },
            None => println!("Key {} not found.", search_key),
        }

        // 4. Contains
        println!("\n--- 4. Checking weather certain node is present using contains ---");
        let keys_to_check = [1, 100];
        for k in keys_to_check {
            let exists = tree::contains(root, k);
            println!("Is key {} present? {}", k, exists);
        }

        // 5. Delete
        println!("\n--- 5. Deleting node using delete method ---");

        println!("Delete node with key = 1");
        tree::delete(&mut root, 1, &mut h);
        tree::print_tree(root, 0);

        println!("Delete node with key 4 (more children)");
        tree::delete(&mut root, 4, &mut h);

        println!("State of tree after deletion:");
        tree::print_tree(root, 0);

        // 6. Free memory
        println!("\n--- 6. Freeing memory ---");
        free_dict(root);
        println!("Memory freed. END");
    }
}