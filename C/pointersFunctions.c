#include <stdio.h>

/*
void findValue (int* num) {
  *num = 39;
}

int main() {

  int num = 21;

  findValue(&num);
  
  printf("%d \n", num);

  return 0;
}
*/ 

int findSquare(int* number) {
  int square = (*number) * (*number);
  return square;
}

int main() {
  int number = 5;

  printf("%d \n", findSquare(&number));
  return 0;
}
