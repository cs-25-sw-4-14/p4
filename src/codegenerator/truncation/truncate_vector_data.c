#include <stdlib.h>
#include <stdint.h>
#include <stdio.h>

void* truncate_vector_data(void* raw_ptr) {
    if (raw_ptr == NULL) {
        printf("Truncate called with NULL pointer\n");
        return NULL;
    }

    uint64_t* header = (uint64_t*) raw_ptr;
    uint64_t size = *header;
    
    printf("Truncating vector: size=%llu, from f64 to f32\n", (unsigned long long)size);
    printf("Old size: %llu bytes, New size: %llu bytes\n", 
           (unsigned long long)(8 + size * sizeof(double)), 
           (unsigned long long)(8 + size * sizeof(float)));

    double* old_data = (double*) (header + 1);

    // Allocate new vector: [size (8 bytes)] + size * sizeof(float)
    size_t new_size = 8 + size * sizeof(float);
    void* new_raw = malloc(new_size);
    if (new_raw == NULL) {
        printf("Truncation malloc failed for %zu bytes\n", new_size);
        return NULL;
    }

    uint64_t* new_header = (uint64_t*) new_raw;
    *new_header = size;

    float* new_data = (float*) (new_header + 1);

    for (uint64_t i = 0; i < size; ++i) {
        new_data[i] = (float) old_data[i];  // Truncate f64 → f32
    }

    printf("Truncation successful, freeing old vector\n");
    free(raw_ptr);  // Free the original vector

    return new_raw;
}
