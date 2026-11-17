#ifndef CUSTOM_AVL_DICTIONARY_H
#define CUSTOM_AVL_DICTIONARY_H

#include <stdint.h>

typedef struct Dict Dict;

Dict* dict(void);

void dict_insert(Dict** root, uint64_t key, const char* value);

const char* dict_find(Dict* root, uint64_t key);

int dict_contains(Dict* root, uint64_t key);

void dict_delete(Dict** root, uint64_t key);

void dict_clear(Dict* root);

#endif