#include <stdio.h>

int main(int argc, char *argv[]) {
  if(argc > 1) {
    printf("Input value: %s\n", argv[1]);
  } else {
    printf("No input value provided\n");
  }

  return 0;
}