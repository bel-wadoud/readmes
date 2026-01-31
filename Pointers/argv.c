#include <stdio.h>

int main(int argc, char **argv) {
  /* argc is the number of stdinputs in terminal (program name counted);
   * **argv is a....
   * okay.. first (*argv) is a list of chars means its a string..
   * *(strings) is a list of strings..
   * means **argv is a list of strings.
   * */

  int j, k; // itterators.

  j = 0;

  while (argv[j]) { // pointers in arguments are treated true if NOT-NULL and
                    // false if NULL
    k = 0;
    while (argv[j][k]) {
      printf("%c \n", argv[j][k]);
      ++k;
    }
    puts("");
    ++j;
  }
}
