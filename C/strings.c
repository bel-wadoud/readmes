#include <stdio.h>
#include <string.h>

int main() {
  char str[] = "abdelwadoud";
  char fullName[30];

  // this function is used to accept a string with spaces.
  fgets(fullName, sizeof(fullName), stdin);
  
  // it shows length + 1 cuz of \0
  printf("%d \n", strlen(fullName));

  // strcpy(dest, source) copies string.
  // strcat(str1, str2) concatinates strings.
  // strcmp(str1, str2) returns 0 if they are equal or random int if not.

  return 0;
}
