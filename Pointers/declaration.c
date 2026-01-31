#include <stdio.h>

int main() {
  int n; // declares an Integer n!
  int *pn; 
  int **ppn;
  int ***pppn;
  
  n = 42; // declares an integer n!
  pn = &n; // points to n's location
  ppn = &pn; // points to pn's location
  pppn = &ppn; // points to ppn's location

  printf("%d \n", n);
  printf("%p \n", pn);
  printf("%p\n", ppn);
  printf("%p\n", pppn);

  printf("%p = %p = %p = %p \n", &n, pn, *ppn, **pppn);

}
