// gcc -o target target.c
#include <stdio.h>
#include <string.h>

int main(int argc, char** argv) {
  char buf[10];

  if(argc < 2) {
    printf("No input provided\n");
    return 1;
  }

  strcpy(buf, argv[1]);

  printf("Input was: %s\n", buf);

  return 0;
}