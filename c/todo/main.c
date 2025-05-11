#include <stdint.h>
#include <stdio.h>

#include "todo/todo.h"

uint8_t manu() {
    printf(
        "What would you like to do?\n"
        "\t1. Add a task\n"
        "\t2. List all tasks\n"
        "\t3. Edit a task\n"
        "\t4. Complete a task\n"
        "\t5. Remove a task\n"
        "\t6. Exit\n"
        "Enter your choice: ");

    uint8_t choice;
    scanf("%hhu", &choice);

    // Consume the remaining newline character
    while (getchar() != '\n');

    return choice;
}

int main(void) {
    print_message("\n\tWelcome to the To-Do List CLI!");

    initialize_tasks();

    while (1) {
        clear_terminal();

        uint8_t choice = manu();

        switch (choice) {
            case 1:
                add_task();
                break;
            case 2:
                list_tasks();
                break;

            case 3:
                edit_task();
                break;
            case 4:
                complete_task();
                break;
            case 5:
                remove_task();
                break;
            case 6:
                exit_todo();
            default:
                print_error("Invalid choice");
                break;
        }
    }

    return 0;
}
