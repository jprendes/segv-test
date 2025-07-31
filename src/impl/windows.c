#include <windows.h>

void try_recover_segv();

static LONG WINAPI segv_handler(struct _EXCEPTION_POINTERS *info) {
    if (info->ExceptionRecord->ExceptionCode != EXCEPTION_ACCESS_VIOLATION) {
        return EXCEPTION_CONTINUE_SEARCH;
    }

    try_recover_segv();

    return EXCEPTION_CONTINUE_SEARCH;
}

void segv_init(void) {
    AddVectoredExceptionHandler(1, segv_handler);
}
