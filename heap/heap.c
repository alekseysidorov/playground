#include <stdio.h>
#include <stdlib.h>

struct leaf_t;
struct leaf_t 
{
    int data;

    struct leaf_t *parent;
    struct leaf_t *left;
    struct leaf_t *right;
};
typedef struct leaf_t leaf_t;

struct heap_t
{
    size_t size;
    struct leaf_t *head;
};
typedef struct heap_t heap_t;

leaf_t *heap_create_leaf(int data, leaf_t *parent)
{
    leaf_t *leaf = (leaf_t*)malloc(sizeof(leaf_t));
    leaf->data = data;
    leaf->parent = parent;
    leaf->left = NULL;
    leaf->right = NULL;
    return leaf;
}

heap_t *heap_create(int data)
{
    heap_t *heap = (heap_t*)malloc(sizeof(heap_t));
    heap->head = heap_create_leaf(data, NULL);
    heap->size = 1;
    return heap;
}

void heap_add_item(heap_t *heap, int data)
{
    int base = 1;
    //int level = 0;
    int s = heap->size;
    
    while (s > 0) {
        s -= base;
        base *= 2;
       // level++;
    }
    leaf_t *item = heap->head;
    if (!s) {
        while (item->left)
            item = item->left;
    } else {
        base /= 2;
        s += base;
        while (base > 2) {
            base /=2;
            if (s >= base) {
                item = item->right;
                s -= base;
            } else 
                item = item->left;
        }
    }
    leaf_t *child = heap_create_leaf(data, item);
    if (!s)
        item->left = child;
    else
        item->right = child;
    heap->size++;

    while (child->parent && child->data < child->parent->data) {
        int tmp = child->data;
        child->data = child->parent->data;
        child->parent->data = tmp;
        child = child->parent;
    }
}

void heap_sort(heap_t *heap)
{
    for (size_t i = 0; i < heap->size; i++) {
        leaf_t *item = heap->head;
        printf("%d ", item->data);
        while (item->left || item->right) {
            if (item->left && (!item->right || item->left->data < item->right->data)) {
                item->data = item->left->data;
                item = item->left;
             } else if (item->right) {
                item->data = item->right->data;
                item = item->right;
            }
        }
        if (item->parent) {
            item = item->parent;
            if (item->left && item->left->data == item->data)
                item->left = NULL;
            else if (item->right)
                item->right = NULL;
        }
    }
}

int main() 
{
    heap_t *heap = heap_create(1);
    heap_add_item(heap, 3);
    heap_add_item(heap, 10);
    heap_add_item(heap, 14);
    heap_add_item(heap, 2);
    heap_add_item(heap, -10);
    heap_add_item(heap, 117);

    heap_sort(heap);

    return 0;
}
