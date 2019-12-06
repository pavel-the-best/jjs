void test_tl()
{
    while(1);
}

void test_il()
{
    usleep(10000000);
}

void test_abort()
{
    abort();
}

void test_return_1()
{
    exit(1);
}

void test_ok()
{
    exit(0);
}

struct test
{
    const char* name;
    void (*func)(void);
};

const struct test tests[] = {
    {"tl", test_tl},
    {"il", test_il},
    {"abort", test_abort},
    {"return1", test_return_1},
    {"ok", test_ok},
};
