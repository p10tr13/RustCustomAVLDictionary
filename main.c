#include <stdio.h>
#include <stdint.h>
#include <inttypes.h>
#include "custom_avl_dictionary.h"

int main(void) {
    printf("=== AVL LIB (C) DEMO ===\n");

    // 1. Creating new dict
    printf("\n--- 1. Creating dictionary ---\n");
    Dict* root = dict();
    printf("Custom AVL dictionary created. Address: %p\n", (void*)root);

    // 2. Insert
    printf("\n--- 2. Insert demo ---\n");

    dict_insert(&root, 1, "a");
    dict_insert(&root, 2, "b");
    dict_insert(&root, 3, "c");
    dict_insert(&root, 4, "d");
    dict_insert(&root, 5, "e");

    printf("Adding a new node with key 6 (rotation is expected)\n");
    dict_insert(&root, 6, "f");
    printf("Updating a value of a node with key 2 using insert operation\n");
    dict_insert(&root, 2, "bb");

    // 3. Find
    printf("\n--- 3. Searching a node using find ---\n");

    uint64_t search_key = 2;
    const char* result = dict_find(root, search_key);
    if (result != NULL) {
        printf("Key %" PRIu64 ": Val -> '%s'.\n", search_key, result);
    } else {
        printf("Key %" PRIu64 " not found.\n", search_key);
    }

    search_key = 100; // Non present in dictionary
    result = dict_find(root, search_key);
    if (result != NULL) {
         printf("Key %" PRIu64 ": Val -> '%s'.\n", search_key, result);
    } else {
         printf("Key %" PRIu64 " not found.\n", search_key);
    }

    // 4. Contains
    printf("\n--- 4. Checking whether a key is present using contains ---\n");

    printf("Is key 1 present?");
    if (dict_contains(root, 1)) {
        printf(" true\n");
    } else {
        printf(" false\n");
    }

    printf("Is key 100 present?");
    if (dict_contains(root, 100)) {
        printf(" true\n");
    } else {
        printf(" false\n");
    }

    // 5. Delete
    printf("\n--- 5. Deleting node using delete method ---\n");

    printf("Delete node (root) with key = 4, with address: %p\n", (void*)root);
    dict_delete(&root, 4);
    printf("Root address after operation: %p\n", (void*)root);

    result = dict_find(root, 4);
    if (result == NULL) {
        printf("Key 4 successfully deleted.\n");
    } else {
        printf("Error occurred while deleting node with key = 4!\n");
    }

    // 6. Free memory
    printf("\n--- 6. Freeing memory ---\n");
    dict_clear(root);
    printf("Memory freed. END\n");

    return 0;
}