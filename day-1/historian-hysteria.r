library(dplyr)

input_file <- "../input/day1.txt"


data <- read.table(input_file, header = FALSE, sep = " ")

df_input <- data %>%
    mutate(first_list = V1, second_list = V4) %>%
    select(first_list, second_list)

first_list_sorted <- df_input %>%
    arrange(first_list) %>%
    .$first_list

second_list_sorted <- df_input %>%
    arrange(second_list) %>%
    .$second_list

df_input_sorted <- data.frame(
    first_list = as.integer(first_list_sorted),
    second_list = as.integer(second_list_sorted)
)

df_distance <- df_input_sorted %>%
    mutate(distance = abs(first_list - second_list)) %>%
    summarise(sum(distance))

head(df_input_sorted)

df_freq <- df_input_sorted %>%
    count(second_list)

df_similarity <- df_input_sorted %>%
    inner_join(df_freq, by = c("first_list" = "second_list")) %>%
    mutate(freq = first_list * n) %>%
    summarise(sum(freq))



head(df_similarity)
