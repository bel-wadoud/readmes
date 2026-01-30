#include <stdio.h>

// structure
enum Size{ 
  Small = 27,
  Medium = 31,
  Large = 35,
  ExtraLarge = 40
};

// short hand
enum Sizes{ 
  Small = 27,
  Medium = 31,
  Large = 35,
  ExtraLarge = 40
} shoesizes;

int main() {

  enum Size shoesize;

  shoesizes = Small;

  shoesize = ExtraLarge;

  printf("%d\n", shoesize);

  return 0;

}
