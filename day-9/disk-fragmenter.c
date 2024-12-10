#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define INPUT_FILE "../input/day9-test.txt"

int checksum(int position, int id) { return position * id; }

void repeat_char(char *c, char digit, int n) {
  for (int i = 0; i < n; i++) {
    c[i] = digit;
  }
  /* printf("%s\n", c); */
}

int main() {
  FILE *file = fopen(INPUT_FILE, "r");
  if (file == NULL) {
    printf("Error: Could not open file %s\n", INPUT_FILE);
    return 1;
  }

  char ch;
  int id = 0, i = 1;
  while ((ch = fgetc(file)) != EOF) {
    /* printf("%c\n", ch); */
    if (atoi(&ch) == 0)
      break;
    char block[atoi(&ch)];
    if (i % 2 != 0) {
      id++;
      repeat_char(block, '0' + id, atoi(&ch));
    } else {
      repeat_char(block, '.', atoi(&ch));
    }
    printf("%s\n", block);
    i++;
  }
  printf("Blocks: %d\n", i - 1);

  fclose(file);

  return 0;
}
