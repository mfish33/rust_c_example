#include <stdint.h>
#include <stdlib.h>

double* generate_data(int size) {
    double* arr = malloc(sizeof(double) * size);
    for(int i=0; i<size; i++) {
        arr[i] = ((double) rand()) / RAND_MAX;
    }

    return arr;
}
