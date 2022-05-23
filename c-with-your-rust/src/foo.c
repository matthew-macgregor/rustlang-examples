#include <stdio.h>
#include "foo.h"

void cool_function(int i, char c, CoolStruct* cs) {
    printf("Hello, world from C!\n");
    printf("cool_function: %d, %c\n", i, c);
    printf("cool_function: x=%d y=%d\n", cs->x, cs->y);
}