# Variables
CC = clang
CFLAGS = -lm -Wall -Wextra -O2 # Compilation flags
SRCS = $(shell find . -name "*.c")  # Find all .c files
OBJS = $(SRCS:.c=) # Replace .c extension with no extension for executables
TESTS = $(shell find . -name "test_*.c")  # Automatically find test source files

# Generate test executable names by removing 'test_' prefix and the directory structure
TEST_EXES = $(patsubst test_%.c, %, $(TESTS))

# Default target to build all executables
all: $(OBJS)

# Rule to compile each .c file into its own executable
%: %.c
	$(CC) $(CFLAGS) -o $@ $<

# Target to build all test executables
%: test_%.c $(OBJS)
	$(CC) $(CFLAGS) -o $@ $< -lcriterion

# Target to run all tests
test: $(TEST_EXES)
	@for exe in $(TEST_EXES); do \
		echo "Running $$exe"; \
		./$$exe; \
	done

# Clean target to remove executables
clean:
	find . -type f -perm /111 -exec rm -f {} +
