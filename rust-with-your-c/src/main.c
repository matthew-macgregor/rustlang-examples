#include "rustlib.h"

/*
* Compile on Windows (MSVC):
* cl .\rust-with-your-c\src\main.c .\target\debug\rust_with_your_c.dll.lib /link /out:.\target\debug\rust-with-your-c.exe
*/

int main(int argc, char **argv) {
    printf("Hello World!\n");
    rust_function();
}