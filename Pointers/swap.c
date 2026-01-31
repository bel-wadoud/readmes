#include <stdio.h>

void swap(int *a, int *b) {
  int swap;
  
  swap = *a;
  *a = *b;
  *b = swap;
}

int main(){
  int a, b;

  a = 5;
  b = 3;

  printf("before: %d %d \n", a, b);
  swap(&a, &b);
  printf("after: %d %d \n", a, b)

  // we wanna swap a and b
  return 0;
}
