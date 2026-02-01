#include <alloca.h>
#include <stdio.h>

// function prototype
int add(int a, int b);
int subtract(int a, int b);
int multiply(int a, int b);
int divide(int a, int b);

int performArithmatic(int a, int b, int (*operation)(int, int)) {
  return operation(a, b);
}

int main() {
  int a = 10, b = 5;

  int sum = performArithmatic(a, b, add);
  int difference = performArithmatic(a, b, subtract);
  int product = performArithmatic(a, b, multiply);
  int quotient = performArithmatic(a, b, divide);

  printf("sum of %d and %d: %d, and sum address %p \n", a, b, sum, add);
  printf("difference between %d and %d: %d, and difference address %p \n", a, b,
         difference, subtract);
  printf("product of %d and %d: %d, and product address %p \n", a, b, product,
         multiply);
  printf("quotient of %d and %d: %d, and quotient address %p \n", a, b,
         quotient, divide);
}

// Functions to perform arithmatic operations
int add(int a, int b) { return a + b; }

int subtract(int a, int b) { return a - b; }

int multiply(int a, int b) { return a * b; }

int divide(int a, int b) { return a / b; }
