#include <signal.h>

void try_recover_segv();

static typeof(void (int)) *PREV_HANDLER;

static void unmask_segv_signal() {
    sigset_t set;
    sigemptyset(&set);
    sigaddset(&set, SIGSEGV);
    sigprocmask(SIG_UNBLOCK, &set, 0);
}

static void segv_handler(int signal) {
    unmask_segv_signal();
    try_recover_segv();
    PREV_HANDLER(signal);
}

void segv_init(void) {
    PREV_HANDLER = signal(SIGSEGV, segv_handler);
}
