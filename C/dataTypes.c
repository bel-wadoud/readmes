#include "stdio.h"

int main() {
  
  int myAge = 25;
  double price = 12.45;
  float number1 = 11.84f; // f is to determine it's a float number
  double expo = 5.5e6; // 5.5^6
  char letter = 'z';


  printf("%d \n", myAge);
  
  printf("the price is %.2lf \n", price);

  printf("number1 is %.2f\n", number1);

  printf(" 5.5 power 6 is: %lf\n", expo);

  printf("letter is %c\n", letter);
  printf("letter %c is DEC is %d\n", letter, letter); // to show that character are represented as digits.
  
  printf("the size of the age variable in bytes is %zubytes \n", sizeof(myAge));

  return 0;
}
