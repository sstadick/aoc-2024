// A cheap and dirty implementation of a Vec
#ifndef VEC_H
#define VEC_H

#include <stdio.h>
#include <stdlib.h>

typedef struct {
  int *data;
  size_t size;
  size_t capacity;
} Vec;

static inline void vec_init(Vec *vec) {
  vec->data = NULL;
  vec->size = 0;
  vec->capacity = 0;
}

static inline void vec_append(Vec *vec, int value) {
  if (vec->size == vec->capacity) {
    // Make it bigger by doubling
    size_t new_capacity = vec->capacity == 0 ? 1 : vec->capacity * 2;
    int *new_data = (int *)realloc(vec->data, new_capacity * sizeof(int));
    if (new_data == NULL) {
      printf("failed to allocate");
      return;
    }
    vec->data = new_data;
    vec->capacity = new_capacity;
  }
  vec->data[vec->size++] = value;
}

static inline void vec_free(Vec *vec) {
  free(vec->data);
  vec->data = NULL;
  vec->size = vec->capacity = 0;
}

static int compare_ints_for_vec(const void *a, const void *b) {
  return (*(int *)a - *(int *)b);
}

static inline void vec_sort(Vec *vec) {
  qsort(vec->data, vec->size, sizeof(int), compare_ints_for_vec);
}

#endif