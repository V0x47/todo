#ifdef _WIN32
#include <windows.h>
#else
#include <unistd.h>
#endif

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "todo.h"

// Static variable to hold the task list
static TaskList *tasks = NULL;

// Global variables
char new_task[MAX_TASK_LEN];
int task_index;

//==============================================================================
// Functions that interact with terminal

void delay() {
#ifdef _WIN32
    Sleep(1000);
#else
    sleep(1);
#endif
}

void clear_terminal() {
#ifdef _WIN32
    system("cls");
#else
    system("clear");
#endif
}

//==============================================================================
// Message functions

// Print message in green color
void print_success(const char *message) {
    printf("\033[1;32m%s\033[0m\n", message);
    delay();
}

// Print message in blue color
void print_message(const char *message) {
    printf("\033[1;34m%s\033[0m\n", message);
    delay();
}

// Print message in red color
void print_error(const char *message) {
    printf("\033[1;31m%s\033[0m\n", message);
    delay();
}

//==============================================================================
// File manipulation functions

// Load tasks from the tasks.csv file
void load_tasks() {
    FILE *fp = fopen("tasks.csv", "r");
    if (fp == NULL) {
        print_error("No tasks available to load");
        exit(1);
    }

    if (ftell(fp) == 0) {
        clear_terminal();
        print_message("There is no task to load in the task list.");
        fclose(fp);
        return;
    }

    char line[MAX_TASK_LEN + 6];
    while (fgets(line, sizeof(line), fp) != NULL) {
        char *token = strtok(line, ",");
        if (token == NULL) {
            continue;
        }

        strncpy(new_task, token, MAX_TASK_LEN - 1);
        new_task[MAX_TASK_LEN - 1] = '\0';

        token = strtok(NULL, ",");
        if (token == NULL) {
            continue;
        }

        int completed = atoi(token);

        if (tasks->index >= MAX_TASKS) {
            print_error("Task list is full");
            break;
        }

        strncpy(tasks->task[tasks->index], new_task, MAX_TASK_LEN - 1);
        tasks->task[tasks->index][MAX_TASK_LEN - 1] = '\0';
        tasks->completed[tasks->index] = completed;
        tasks->index++;
    }

    fclose(fp);
}

// Save tasks to the tasks.csv file
void save_tasks() {
    FILE *fp = fopen("tasks.csv", "w");
    if (fp == NULL) {
        print_error("Could not open tasks.csv for writing");
        exit(1);
    }

    for (int i = 0; i < tasks->index; i++) {
        fprintf(fp, "%s,%d\n", tasks->task[i], tasks->completed[i]);
    }

    fclose(fp);
}

//==============================================================================
// Supporting functions for the To-Do List CLI

void print_tasks() {
    for (int i = 0; i < tasks->index; i++) {
        printf("%d: [%c] %s\n", i + 1, tasks->completed[i] ? 'X' : ' ',
               tasks->task[i]);
    }
}

uint8_t get_index(const char *message) {
    if (tasks->index == 0) {
        char msg[32];
        sprintf(msg, "No tasks available to %s!\n", message);
        print_message(msg);
        return 1;
    }

    printf("Available tasks:\n\n");
    print_tasks();
    printf("\nEnter the index of the task to %s (0 to abort): ", message);

    if (scanf("%d", &task_index) != 1 || task_index > tasks->index ||
        task_index < 0) {
        print_error("Invalid input");
        while (getchar() != '\n');
        return 1;
    }
    while (getchar() != '\n');

    // Print "Action aborted" when task_index is 0 and return 1
    if (task_index == 0) {
        print_message("Action aborted");
        return 1;
    }

    return 0;
}

//==============================================================================
// Task list functions

void initialize_tasks() {
    if (tasks == NULL) {
        tasks = (TaskList *)malloc(sizeof(TaskList));
        if (tasks == NULL) {
            print_error("Memory allocation failed");
            exit(1);
        }
        tasks->index = 0;
    }

    load_tasks();
}

void add_task() {
    clear_terminal();
    printf("Enter a new task: ");

    if (fgets(new_task, sizeof(new_task), stdin) != NULL) {
        new_task[strcspn(new_task, "\n")] = '\0';
        if (strlen(new_task) == 0 || strlen(new_task) >= MAX_TASK_LEN) {
            print_error("Invalid task");
            return;
        }

        if (tasks->index >= MAX_TASKS) {
            print_error("Task list is full");
            return;
        }

        strncpy(tasks->task[tasks->index], new_task, MAX_TASK_LEN - 1);
        tasks->task[tasks->index][MAX_TASK_LEN - 1] = '\0';
        tasks->completed[tasks->index] = 0;
        tasks->index++;
        print_success("Task added");
    }
}

// Display all tasks in the task list
void list_tasks() {
    clear_terminal();

    if (tasks->index == 0) {
        print_message("No tasks available.");
    } else {
        printf("Tasks:\n\n");
        print_tasks();
        printf("\n\nPress any key to continue...");
        getchar();
    }
}

void edit_task() {
    // Let user choose which task to edit
    clear_terminal();
    if (get_index("edit") == 1)
        return;

    // Prompt the user to enter a new task
    clear_terminal();
    printf("Enter a new task: ");
    if (fgets(tasks->task[task_index - 1], MAX_TASK_LEN, stdin) != NULL) {
        // Remove the newline character if present
        tasks
            ->task[task_index - 1][strcspn(tasks->task[task_index - 1], "\n")] =
            '\0';
        print_success("Task edited");
    } else {
        print_error("Failed to read new task input");
    }

    // If the task is completed, ask the user whether they want to mark it as uncompleted
    clear_terminal();
    if (tasks->completed[task_index - 1] == 1) {
        char answer;
        while (1) {
            print_message(
                "Task is already completed. Do you want to change it to "
                "uncompleted? (y/n): ");

            // Take input from user
            scanf(" %c", &answer);

            switch (answer) {
                case 'y':
                case 'Y':
                    tasks->completed[task_index - 1] = 0;
                    clear_terminal();
                    print_success("Task is now uncompleted");
                    return;
                case 'n':
                case 'N':
                    clear_terminal();
                    print_success("Task stays completed");
                    return;
                default:
                    print_error("Invalid input. Please enter 'y' or 'n'.");
                    clear_terminal();
            }
        }
    }
}

void complete_task() {
    clear_terminal();
    if (get_index("complete") == 1)
        return;
    tasks->completed[task_index - 1] = 1;
    print_success("Task completed");
}

void remove_task() {
    clear_terminal();
    if (get_index("remove") == 1)
        return;

    for (int i = task_index - 1; i < tasks->index - 1; i++) {
        strncpy(tasks->task[i], tasks->task[i + 1], MAX_TASK_LEN);
        tasks->completed[i] = tasks->completed[i + 1];
    }
    memset(tasks->task[tasks->index - 1], 0, MAX_TASK_LEN);
    tasks->index--;
    print_success("Task removed");
}

void exit_todo() {
    clear_terminal();
    save_tasks();
    free(tasks);
    tasks = NULL;
    print_message("Exiting...");
    clear_terminal();
    exit(0);
}
