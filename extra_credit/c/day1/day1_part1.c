#include "pair.h"
#include "vec.h"
#include <stdio.h>
#include <stdlib.h>

DEFINE_PAIR(Vec, Vec)

Pair_Vec read_numbers(const char *filename) {
  FILE *file = fopen(filename, "r");
  if (!file) {
    perror("Error opening file");
    exit(1);
  }
  Vec a_numbers, b_numbers;
  int a, b;
  vec_init(&a_numbers);
  vec_init(&b_numbers);

  while (fscanf(file, "%d %d", &a, &b) == 2) {
    vec_append(&a_numbers, a);
    vec_append(&b_numbers, b);
  }

  Pair_Vec result;
  result.a = a_numbers;
  result.b = b_numbers;
  vec_sort(&result.a);
  vec_sort(&result.b);

  return result;
}

int main() {
  Pair_Vec numbers = read_numbers("input.txt");

  int total = 0;
  for (int i = 0; i < numbers.a.size; i++) {
    int a = numbers.a.data[i];
    int b = numbers.b.data[i];
    int diff = abs(a - b);
    total += diff;
  }
  vec_free(&numbers.a);
  vec_free(&numbers.b);
  printf("Answer: %d\n", total);

  return 0;
}