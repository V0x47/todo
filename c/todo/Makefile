CC = clang
CFLAGS = -Wall -Wextra -Werror -std=c11
SRC = main.c todo/todo.c
OBJ = $(SRC:.c=.o)
TARGET = todo_exec  # Change this to avoid overwriting the directory

# Directories (if you have header files in a separate directory)
INCLUDES = -I./todo

# Compile all
all: $(TARGET)

# Linking object files to create the executable
$(TARGET): $(OBJ)
	$(CC) $(CFLAGS) $(INCLUDES) -o $(TARGET) $(OBJ)

# Pattern rule to compile .c to .o
%.o: %.c
	$(CC) $(CFLAGS) $(INCLUDES) -c $< -o $@

# Clean up the generated files
clean:
	rm -f $(OBJ) $(TARGET)

.PHONY: all clean

