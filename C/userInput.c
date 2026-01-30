#include "stdio.h"

int main() {
  
  int age;
  char charachter;

  printf("enter my age\n");
  scanf("%d", &age); // & specifies the memory allocation.
  
  printf("%d\n", age);

  // asking for multiple input
  printf("enter ur data: \n");
  scanf("%d %c", &age, &charachter);
  return 0;
}
