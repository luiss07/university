#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
    char input[64];

    setbuf(stdout, NULL);
    setbuf(stdin, NULL);
    setbuf(stderr, NULL);

    puts("Hello dear user! What's your name?");
    fgets(input, 64, stdin);
    printf("Hello, ");
    printf(input);
    printf("\n");

    while (1) {
        printf("What feature do you want me to explain?\n");
        fgets(input, 64, stdin);
        printf("You want me to explain the feature: ");
        printf(input);
        printf("\n");

        if (memcmp(input, "shell", 5) == 0) {
            printf("Well, this one is easy, you just need to run: ");
            puts("/bin/bash");
        } else if (memcmp(input, "flag", 4) == 0) {
            printf("To get the flag you need to run: ");
            puts("cat flag.txt");
        } else if (memcmp(input, "exit", 4) == 0) {
            break;
        } else {
            printf("Sorry, I don't know that feature\n");
        }
    }

    printf("Goodbye!\n");
    return 0;
}