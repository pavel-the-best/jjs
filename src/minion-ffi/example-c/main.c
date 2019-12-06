#include <jjs/minion-ffi.h>
#include <unistd.h>
#include "stdio.h"
#include "assert.h"

const char* MSG_FALLTHROUGH = "unknown error kind";
const char* MSG_INVALIDINPUT = "invalid input";
const char* MSG_UNKNOWN = "unknown error in minion-ffi";

void error_check(int err) {
    if (err == ERROR_CODE_OK) return;
    const char* msg = MSG_FALLTHROUGH;
    if (err == ERROR_CODE_INVALID_INPUT) {
        msg = MSG_INVALIDINPUT;
    } else if (err == ERROR_CODE_UNKNOWN) {
        msg = MSG_UNKNOWN;
    }

    fprintf(stderr, "minion-ffi error %d (%s)\n", err, msg);
    exit(1);
}

int main() {
    int status;
    status = minion_lib_init();
    error_check(status);
    struct Minion_Backend* backend;
    status = minion_backend_create(&backend);
    error_check(status);
    struct Minion_DominionOptions dopts = {
        .time_limit = {1, 0},
        .process_limit = 16,
        .memory_limit = 0x40000000,
        .isolation_root = "/tmp/is",
        .shared_directories = (struct Minion_SharedDirectoryAccess[2]){
            {SHARED_DIRECTORY_ACCESS_KIND_READONLY, "/bin", "/bin"},
            SHARED_DIRECTORY_ACCESS_FIN,
        },
    };
    struct Minion_Dominion* dominion;
    status = minion_dominion_create(backend, dopts, &dominion);
    error_check(status);
    struct Minion_ChildProcessOptions cpopts = {
        .image_path = "/bin/busybox",
        .argv = (char*[3]){"ls", "/proc", NULL},
        .envp = (struct Minion_EnvItem[1]){ENV_ITEM_FIN},
        .stdio = {0, 1, 2},
        .dominion = dominion,
        .workdir = "/",
    };
    struct Minion_ChildProcess* cp;
    status = minion_cp_spawn(backend, cpopts, &cp);
    error_check(status);
    Minion_WaitOutcome outcome;
    status = minion_cp_wait(cp, &(struct Minion_TimeSpec){5, 0}, &outcome);
    error_check(status);
    if(status == WAIT_OUTCOME_TIMEOUT)
    {
        status = minion_cp_kill(cp);
        error_check(status);
    }
    status = minion_cp_free(cp);
    error_check(status);
    status = minion_dominion_free(dominion);
    error_check(status);
    status = minion_backend_free(backend);
    error_check(status);
}
