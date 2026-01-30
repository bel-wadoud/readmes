#include <stdio.h>

int main() {
  printf("data is converted from lower to upper depending on this diagram \n long double \n double \n float \n long \n int \n short \n char \n");
    
  double a = 5.67;
  
  int result0 = a + 4;
  double result1 = (int)a + 4; // (int) will convert a explicitly to an integer.

  printf("%d %lf \n", result0, result1);
  return 0;
}
