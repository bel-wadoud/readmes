#include <stdio.h>
#include <stdlib.h>

/*
 * malloc(N); // give me raw memory of size N
 *
 * calloc(5, sizeof(int)); // give me clean memory, for 5 elements of size int.
 * (it initializes everything to zero!)
 *
 * realloc(ptr, sizeof(N)); // resize this ptr
 *
 * free(ptr) // returns memory to the OS
 */

int main() {
  int arr[10];

  arr[0] = 14;

  // give me space for 3 integers in memory
  int *mall = malloc(3 * sizeof(int));

  if (mall == NULL) {
    printf("allocation failed. \n");

    return 0;
  }

  int *real = realloc(mall, 4 * sizeof(int));
  if (real == NULL) {
    // p is STILL VALID
    // you didn’t lose it
  } else {
    // mall = real;
  }

  /* the name of an array is a synonym for the location of the 1st element
   */
  printf("%p, %p \n", &arr, &arr[0]);

  printf("the Address of the original array is: \n == %p ==\n ", &arr);

  // fake array
  int *ptr = alloca(sizeof(int) * 10);

  *(ptr + 0) = 10;
  *(ptr + 1) = 14;
  *(ptr + 2) = -31;

  printf("fake array elements! \n");
  for (int i = 0; i < 10; ++i) {
    printf("%d \n", *(ptr + i));
  }

  return 0;
}
