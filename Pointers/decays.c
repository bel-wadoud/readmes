#include <stdio.h>

int main() {

  int arr[10] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 99};
  for (int i = 0; i < 10; ++i) {
    printf("%d \n", i[arr]); // LIKE SEEEE WTF IS THIIIISISISSISSSSSS.
    /* this basically wroks cuz it's equivalent to *(arr + i) which is basically
     * *(i + arr) */
  }

  // u can treat strings like a list of chars too!
  printf("%c \n", "Hello"[0]);

  for (int i = 0; i < 5; ++i) {
    printf("%c \n", "Hello"[i]);
    printf("%c \n", i["Hello"]); // enjoying life :)
  }
  return 0;
}
