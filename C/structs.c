#include <stdio.h>


// structure
struct Person {
  int age;
  double salary;
};

// to shorting calling structs
typedef struct Game {
  int rate;
  int reviews;
} game;


int main() {
  
  struct Person Person1;
  game game1 = {.rate = 10, .reviews = 11};

  Person1.age = 25;
  Person1.salary = 43121.21;

  // short hand
  struct Person Person2 = {.age = 21, .salary = 1348.12};

  printf("%d %.2lf \n", Person1.age, Person1.salary);

  return 0;


}
