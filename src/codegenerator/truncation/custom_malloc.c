#include <stdlib.h>
#include <stddef.h>
#include <stdio.h>

#define MAX_ALLOC_BYTES  (1024)

void *custom_malloc(size_t size) {
    if (size > MAX_ALLOC_BYTES) {
        printf("Malloc failed: requested %zu bytes, limit is %d bytes\n", size, MAX_ALLOC_BYTES);
        return NULL;
    }
    void *ptr = malloc(size);
    if (ptr == NULL) {
        printf("System malloc failed for %zu bytes\n", size);
    }
    return ptr;
}
