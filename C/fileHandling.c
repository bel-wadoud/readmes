#include <stdio.h>


int main() {
  FILE* fptr;

  // read contents from a file

  fptr = fopen("test.txt", "r");

  char content[1000];
  
  if (fptr != NULL) {
    while(fgets(content, 1000, fptr);) {
      printf("%s\n", content);
    }
  } else {
    printf("file opened unseccessfully.\n");
  }

  fclose();

  // write contents to a file .
  fptr = fopen("newfile.txt", "w");

  fputs("This is a simpler printf.\n", fptr);

  return 0;
}

