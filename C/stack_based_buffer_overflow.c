#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <string.h>
#include <ctype.h>

void return_last(char* input) {
    char dest[8]={0};
    memcpy(dest, input+8, sizeof(input));
    printf("%s\n", dest);
}

void overwrite_constant(char* input) {
    char const constant[3] = "ABC";
    char buffer[8];
    printf("Konstante %s\n", constant);
    strcpy(buffer, input);
    printf("Konstante %s\n", constant);
}

void vulnerable(char* input) {
    char same_frame[8] = "AAAAAAAA"; // Can be overwritten
    printf("(vuln_func) same_frame: %s \n\n", same_frame);
    printf("(vuln_func) input: %s \n", input);
    char buffer[8];
    printf("Stackbasierter Bufferoverflow\n");
    strcpy(buffer, input); // Buffer overflow on the stack

    //WTF
    printf("same_frame: %s \n", same_frame);

    overwrite_constant("BUFFEROVERFLOW");
    printf("Letztes Zeichen ddes same_frames: ");
    return_last(same_frame);
}

void parent() {
    char parent_frame[8] = "BBBBBBBB"; // Also overwritten
    printf("(parent_func) parent_frame: %s \n", parent_frame);
    vulnerable("BUFFEROVERFLOW");
    // Dangerous if parent_frame is passed, e.g., to exec
}

int main(void) {
    parent();
}