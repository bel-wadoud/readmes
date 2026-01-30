#include <stdio.h>
// to use external built-in functions we need to import their lib
#include <math.h>

// to declare that a function has no return we use void
void greet() {
  printf("Good Morning!\n");
}

// function with input parameters
void calc(int number) {
  int result = number * number;
  printf("%d \n", result);
}

void sum(int number1, int number2) {
  int result = number1 + number2;
  printf("%d \n", result);
}

// to declare a function that has a return
int sum2(int number1, int number2) {
  int result = number1 + number2;
  return result;
}


// to use a function that is declared after main function we need to define it's prototype
int sum3(int number1, int number2);

int main () {
  // calling the function greet
  greet();

  calc(4);

  printf("sum\n");
  sum3(5, 2);

  return 0;
}


// to declare a function that has a return
int sum3(int number1, int number2) {
  int result = number1 + number2;
  return result;
}
