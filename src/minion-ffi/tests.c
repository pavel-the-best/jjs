#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <assert.h>

#include "tests.h"

int main(int argc, char** argv)
{
    if(argc != 2)
        abort();
    for(int i = 0; i < sizeof(tests) / sizeof(tests[0]); i++)
        if(!strcmp(tests[i].name, argv[1]))
        {
            tests[i].func();
            assert(!"program has not exited after running test");
        }
    return 179;
}
