#include <stdio.h>

int main() {
  int age = 25;

  printf("%p \n", &age); // %p means the memory location.

  // declaring a pointer
  int* ptr = &age;

  printf("%p \n", ptr); 

  // to print the value of a given Memory location.
  printf("%d \n", *ptr);

  // assign a new value to a pointer
  
  *ptr = 31;

  printf("%d\n", age);

  return 0;
}
