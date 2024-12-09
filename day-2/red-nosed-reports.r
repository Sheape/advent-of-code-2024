## library(dplyr)

input_data <- "../input/day2.txt"

lines <- readLines(input_data)

level_change <- function(levels) {
    level_chg <- c()
    for (i in seq_along(levels)) {
        level_chg[i] <- levels[i] - levels[i + 1]
    }
}

for (line in lines) {
    input_line <- unlist(strsplit(line, " "))
    print(length(input_line) - 1)
}
