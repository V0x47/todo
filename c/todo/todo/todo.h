#ifndef TODO_H
#define TODO_H

#include <stdint.h>

// Constants
#define MAX_TASKS 256
#define MAX_TASK_LEN 100

// Task list
typedef struct {
    char task[MAX_TASKS][MAX_TASK_LEN];
    uint8_t completed[MAX_TASKS];
    uint16_t index;
} TaskList;

// Print messages to the console
void print_success(const char *message);
void print_message(const char *message);
void print_error(const char *message);

// Clear the terminal
void clear_terminal();

// Initialize the task list
void initialize_tasks();

// Edit the task list
void add_task();
void list_tasks();
void edit_task();
void complete_task();
void remove_task();

// Exit the program
void exit_todo();

#endif  // TODO_H
