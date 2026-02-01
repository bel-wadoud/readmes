#include <stdio.h>

typedef union {
  float price;
  int quantity;
  int full_price;
} product_t;

int main() {

  printf("size of this union is: %zu \n",
         sizeof(product_t)); // prints the bigger value

  product_t tomato = {.price = 15.00};

  printf("price: %f \n", tomato.price);

  tomato.quantity = 4;

  printf("price %f, quantity %d \n", tomato.price, tomato.quantity);

  return 0;
}
