#include <setjmp.h>

static _Thread_local struct {
    jmp_buf buf;
    int is_some;
} SEGV_JUMP_BUF;

void try_recover_segv() {
    if (SEGV_JUMP_BUF.is_some) {
        SEGV_JUMP_BUF.is_some = 0;
        longjmp(SEGV_JUMP_BUF.buf, 42);
    }
}

int check_segv(void (*f)(void *), void *arg) {
    if (SEGV_JUMP_BUF.is_some) {
        return 2;
    }

    SEGV_JUMP_BUF.is_some = 1;
    if (setjmp(SEGV_JUMP_BUF.buf) != 0) {
        return 1;
    }

    f(arg);
    
    SEGV_JUMP_BUF.is_some = 0;
    return 0;
}