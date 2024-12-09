void swap(int *a, int *b) {
  int temp = *a;
  *a = *b;
  *b = temp;
}

int quick_sort(int arr[], int start, int end) {
  int pivot = arr[end];
  int i = start - 1;
  for (int j = start; j < end; j++) {
    if (arr[j] <= pivot) {
      i++;
      swap(&arr[i], &arr[j]);
    }
  }
  swap(&arr[i + 1], &arr[end]);

  return i + 1;
}

void quick_sort_merged(int arr[], int start, int end) {
  if (start < end) {
    int pivot_index = quick_sort(arr, start, end);
    quick_sort_merged(arr, start, pivot_index - 1);
    quick_sort_merged(arr, pivot_index + 1, end);
  }
}
