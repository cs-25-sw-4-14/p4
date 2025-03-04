#include <stdio.h>
#include <stdlib.h>
#include <sys/resource.h>
#include <limits.h>

float** truncToFloatArray(double** x, size_t size) {
    if (size == 0) return NULL;

    float** y = malloc(sizeof(float*));
    if (!y) {
        perror("Failed to allocate float**");
        exit(1);
    }
    y[0] = malloc(sizeof(float));
    *y[0] = (float) *x[0];
    free(x[0]);

    for (size_t i = 1; i < size; i++) {
        float** temp = realloc(y, sizeof(float*) * (i+1));
        if(!temp) {
            perror("Failed to realloc float**");
            exit(1);
        }
        y = temp;
        y[i] = malloc(sizeof(float));
        if (!y[i]) {
            perror("Failed to allocate float*");
            exit(1);
        }
        *y[i] = (float) *x[i];
        free(x[i]);
    }

    free(x);
    return y;
}

int main() {
    //Since limits don't work on mac, manually keep track:
    const long lim = (long) 512*1024*1024;
    
    long amt_doubles = 10000;
    double **x = malloc(sizeof(double *) * amt_doubles);
    if (!x) {
        perror("Initial malloc failed");
        return 1;
    }
    
    for (int i = 0; i < amt_doubles; i++) {
        x[i] = malloc(sizeof(double));
        if (!x[i]) {
            perror("malloc failed for double");
            for (int j = 0; j < i; j++) {
                free(x[j]);
            }
            free(x);
            return 1;
        }
        *x[i] = 2222.6 / (i + 1);
    }
    
    printf("Mallocing until out of memory...\n");
    
    long initial_limit = 0;
    long double_size = (amt_doubles * sizeof(double *)) + (amt_doubles * sizeof(double));
    do {
        long old_size = amt_doubles;
        amt_doubles *= 1.1;
        
        double **temp = realloc(x, sizeof(double *) * amt_doubles);
        if (!temp) {
            perror("realloc failed");
            printf("Hit memory limit during realloc at %ld doubles\n", amt_doubles);
            amt_doubles = old_size;
            break;
        }
        
        x = temp;
        
        long i;
        for (i = old_size; i < amt_doubles; i++) {
            x[i] = malloc(sizeof(double));
            if (!x[i]) {
                printf("Hit memory limit at %ld doubles\n", i);
                for(int j = old_size; j <= i; j++){
                    free(x[j]);
                }
                break;
            }
            *x[i] = 2222.6 / (i + 1);
        }
        double_size = (amt_doubles* sizeof(double *)) + (amt_doubles* sizeof(double));

        if(i < amt_doubles) {
            amt_doubles = old_size;
            break;
        }
    } while (double_size < lim);

    initial_limit = amt_doubles;
    
    printf("\nMaximum doubles allocated: %ld\n\n", amt_doubles);
    
    printf("First and last double:\n");
    printf("x[%d] = %.50f (%lu bytes) at address %p\n", 0, *x[0], sizeof(*x[0]), x[0]);
    printf("x[%ld] = %.50f (%lu bytes) at address %p\n", amt_doubles-1, *x[amt_doubles-1], sizeof(*x[amt_doubles-1]), x[amt_doubles-1]);
    
    printf("\nConverting doubles to floats\n");

    float **y = truncToFloatArray(x, amt_doubles);

    printf("\nMemory usage statistics:\n");
    long float_size = (amt_doubles * sizeof(float *)) + (amt_doubles * sizeof(float));
    
    printf("Original memory for doubles: %zu bytes (%zu KB)\n", double_size, double_size / 1024);
    printf("New memory after conversion: %zu bytes (%zu KB)\n", float_size, float_size / 1024);
    printf("Memory saved: %zu bytes (%zu KB)\n", double_size - float_size, (double_size - float_size) / 1024);
    
    printf("\nFirst and last floats:\n");
    printf("x[%d] = %.50f (%lu bytes) at address %p\n", 0, *y[0], sizeof(*y[0]), y[0]);
    printf("x[%ld] = %.50f (%lu bytes) at address %p\n", amt_doubles-1, *y[amt_doubles-1], sizeof(*y[amt_doubles-1]), y[amt_doubles-1]);
    
    printf("\nNow trying to allocate more elements as floats...\n");
    
    do {
        long old_size = amt_doubles;
        amt_doubles *= 1.1;
        
        float **new_y = (float**)realloc(y, sizeof(float*) * amt_doubles);
        if (!new_y) {
            printf("Realloc failed at %ld elements\n", amt_doubles);
            break;
        }
        
        y = new_y;
        
        long i;
        for (i = old_size; i < amt_doubles; i++) {
            y[i] = (float*)malloc(sizeof(float));
            if (!y[i]) {
                printf("Hit memory limit at %ld elements\n", i);
                break;
            }
            *y[i] = (float)(2222.6 / (i + 1));
        }
        
        if (i < amt_doubles) {
            amt_doubles = i-1;
            break;
        }
        
        float_size = (amt_doubles * sizeof(float *)) + (amt_doubles * sizeof(float));
    } while (float_size < lim);
    
    printf("\nFinal statistics:\n");
    printf("Initial limit with doubles: %ld\n", initial_limit);
    printf("Final count with floats: %ld\n", amt_doubles);
    printf("Additional elements allocated: %ld\n", amt_doubles - initial_limit);
    printf("Percentage increase: %.2f%%\n", (amt_doubles - initial_limit) * 100.0 / initial_limit);
    
    for (int i = 0; i < amt_doubles; i++) {
        free(y[i]);
    }
    free(y);
    
    return 0;
}
