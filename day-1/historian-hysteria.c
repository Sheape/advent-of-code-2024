#include "../include/quicksort.h"
#include <stdio.h>
#include <stdlib.h>

#define INPUT_FILE "../input/day1.txt"
#define NO_OF_LOCATIONS 1000

int calc_distance(int a, int b) { return abs(a - b); }

void get_locations(int *arr1, int *arr2) {
  FILE *file = fopen(INPUT_FILE, "r");
  if (file == NULL) {
    printf("Error: Could not open file %s\n", INPUT_FILE);
  }

  char line[64];
  int i = 0;
  while (fgets(line, sizeof(line), file) != NULL) {
    int location_id_1, location_id_2;
    fscanf(file, "%d    %d", &location_id_1, &location_id_2);
    arr1[i] = location_id_1;
    arr2[i] = location_id_2;
    i++;
  }
  fclose(file);
}

int main() {
  int first_list[NO_OF_LOCATIONS];
  int second_list[NO_OF_LOCATIONS];
  printf("first_list addr: %p\n", first_list);
  printf("second_list addr: %p\n", second_list);

  get_locations(first_list, second_list);

  quick_sort_merged(first_list, 0,
                    sizeof(first_list) / sizeof(first_list[0]) - 1);
  quick_sort_merged(second_list, 0,
                    sizeof(second_list) / sizeof(second_list[0]) - 1);

  int total_distance = 0;

  for (int i = 0; i < NO_OF_LOCATIONS; i++) {
    int distance = calc_distance((int)first_list[i], (int)second_list[i]);
    printf("%d - %d = %d\n", first_list[i], second_list[i], distance);
    total_distance += distance;
  }
  printf("Total Distance: %d\n", total_distance);
  return 0;
}
