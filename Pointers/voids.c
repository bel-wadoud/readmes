#include <stdio.h> 

void printData(void *data, char dataType) {
  switch (dataType) {
    case 'i':
      printf("%d \n", *((int*) data));
      break;
    case 'c':
      printf("%c \n", *((char*) data));
      break;
    case 'f':
      printf("%f \n", *((float*) data));
      break;
    default:
      printf("wrong input.\n");
  }
}

int main() {
  int data = 25;

  printData(&data, 'i');

  return 0;
}
