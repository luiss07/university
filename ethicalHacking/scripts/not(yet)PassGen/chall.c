#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int admin = 0;

void win() {
    FILE *f = fopen("flag.txt", "r");
    char flag[64];
    fgets(flag, sizeof(flag), f);
    printf("Hello, admin\nHere is your flag: %s", flag);
    exit(0);
}

int main() {
    char name[64];

    setbuf(stdout, NULL);
    setbuf(stdin, NULL);
    setbuf(stderr, NULL);

    printf("Welcome to Password Generator that does not even generate a password!\n\n");
    
    for(int i = 0; i < 4; i++) {
        printf("username: ");
        fgets(name, sizeof(name), stdin);

        if (name[strlen(name) - 1] == '\n') {
            name[strlen(name) - 1] = '\0';
        }

        printf("Hello, ");
        printf(name);

        printf("\nAdmin: 0x%08x\n", admin);

        if (admin == 0xdeadbeef) {
            win();
        } else {
            puts("Nope.\n");
        }
    }

    return 0;
}