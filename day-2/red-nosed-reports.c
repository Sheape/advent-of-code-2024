#include <stdio.h>

#define NO_OF_LEVELS 100
#define INPUT_FILE "../input/day2.txt"



int main() {
  FILE *file = fopen(INPUT_FILE, "r");
  char buffer[20];

  for (int i = 0; i < NO_OF_LEVELS; i++) {
    fgets(buffer, sizeof(buffer), file);
  }

  fclose(file);
}
