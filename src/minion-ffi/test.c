#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/wait.h>
#include <assert.h>
#include <jjs/minion-ffi.h>

#define __call_where(f, l, s, ...) f(__FILE__ ":" #l ": `" #s "`", __VA_ARGS__)
#define _call_where(f, l, ...) __call_where(f, l, f(__VA_ARGS__), __VA_ARGS__)
#define call_where(f, ...) _call_where(f, __LINE__, __VA_ARGS__)

static inline void verify_ok(char* where, Minion_ErrorCode code)
{
    if(code == ERROR_CODE_INVALID_INPUT)
    {
        fprintf(stderr, "%s failed: Invalid input\n", where);
        abort();
    }
    else if(code != ERROR_CODE_OK)
    {
        fprintf(stderr, "%s failed: Unknown\n", where);
        abort();
    }
}

#define verify_ok(...) call_where(verify_ok, __VA_ARGS__)

static inline void assert_write(char* where, int fd, char* buf, int count)
{
    if(write(fd, buf, count) != count)
    {
        fprintf(stderr, "%s: write failed: ", where);
        perror("");
        abort();
    }
}

#define assert_write(...) call_where(assert_write, __VA_ARGS__)

void run_test(const char* self, const char* dir, const char* param)
{
    int devnull_fd = open("/dev/null", O_RDWR);
    verify_ok(minion_lib_init());
    struct Minion_Backend* bk;
    verify_ok(minion_backend_create(&bk));
    struct Minion_Dominion* minion;
    verify_ok(minion_dominion_create(bk, (struct Minion_DominionOptions){
        .time_limit = {1, 0},
        .process_limit = 1,
        .memory_limit = 0x40000000,
        .isolation_root = dir,
        .shared_directories = (struct Minion_SharedDirectoryAccess[3]){
             {SHARED_DIRECTORY_ACCESS_KIND_READONLY, self, "/me"},
             {SHARED_DIRECTORY_ACCESS_KIND_READONLY, "/bin/busybox", "/bin/busybox"},
             {0, NULL, NULL},
        },
    }, &minion));
    struct Minion_ChildProcess* proc;
    verify_ok(minion_cp_spawn(bk, (struct Minion_ChildProcessOptions){
        .image_path = "/me",
        .argv = (char*[]){(char*)param},
        .envp = (struct Minion_EnvItem[1]){ENV_ITEM_FIN},
        .stdio = {devnull_fd, dup(1), dup(1)},
        .dominion = minion,
        .workdir = "/"
    }, &proc));
    Minion_WaitOutcome outcome;
    verify_ok(minion_cp_wait(proc, &(struct Minion_TimeSpec){100, 0}, &outcome));
    if(outcome == WAIT_OUTCOME_TIMEOUT)
    {
        verify_ok(minion_cp_kill(proc));
        assert_write(1, "ILE\n", 4);
    }
    else if(outcome == WAIT_OUTCOME_ALREADY_FINISHED)
        assert_write(1, "Already finished, WTF?\n", 23);
    else
    {
        int64_t exitcode = 57179444;
        verify_ok(minion_cp_exitcode(proc, &exitcode));
        char data[64];
        assert_write(1, data, sprintf(data, "exit code %lld\n", (long long)exitcode));
    }
    verify_ok(minion_cp_free(proc));
    verify_ok(minion_dominion_free(minion));
    verify_ok(minion_backend_free(bk));
    exit(0);
}

#include "tests.h"

int main(int argc, const char** argv)
{
    char self[1024];
    int sz = readlink("/proc/self/exe", self, 1022);
    assert(sz >= 0 || !"readlink failed");
    self[sz] = 0;
    if(argc == 3)
    {
        self[sz] = 's';
        self[sz+1] = 0;
        run_test(self, argv[1], argv[2]);
        assert(!"program has not exited during run_test");
    }
    else if(argc == 1)
    {
        int have_fails = 0;
        for(int i = 0; i < sizeof(tests) / sizeof(tests[0]); i++)
        {
            fprintf(stderr, "running `%s`\n", tests[i].name);
            char tempdir[] = "/tmp/tmpXXXXXX";
            assert(mkdtemp(tempdir));
            int pid = fork();
            if(!pid)
            {
                execl(self, self, tempdir, tests[i].name, NULL);
                perror("execl");
                abort();
            }
            int status;
            assert(wait(&status) == pid);
            if(!WIFEXITED(status) || WEXITSTATUS(status))
            {
                fprintf(stderr, "*** FATAL ERROR: test `%s`: run_test aborted. Please clean the tempdir (%s) MANUALLY! ***\n", tests[i].name, tempdir);
                abort();
            }
            fprintf(stderr, "test `%s` OK\n", tests[i].name);
            pid = fork();
            if(!pid)
            {
                execlp("rm", "rm", "-r", tempdir, NULL);
                perror("rm -rf $tempdir");
                abort();
            }
            assert(wait(&status) == pid);
            assert(WIFEXITED(status) && !WEXITSTATUS(status) || !"rm -rf $tempdir");
        }
        if(!have_fails)
            fprintf(stderr, "all OK\n");
        return have_fails;
    }
    else
    {
        puts("usage: sudo %s\n\nRun minion-ffi tests.");
        return 2;
    }
}
