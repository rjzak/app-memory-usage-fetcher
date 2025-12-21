// SPDX-License-Identifier: Apache-2.0

#if defined(__APPLE__) || defined(__linux__) || defined(__FreeBSD__) || defined(__OpenBSD__) || defined(__NetBSD__) || defined(__DragonFly__) || defined(_AIX) || defined(__ANDROID__) || defined(__sun)
#include <sys/resource.h>

long getMemoryUsage() {
    struct rusage usage;
    if(0 == getrusage(RUSAGE_SELF, &usage))
        #if defined(__sun) || defined(__APPLE__)
            return usage.ru_maxrss; // Solaris, macOS reports kilobytes
        #else
            return usage.ru_maxrss * 1024; // kilobytes to bytes
        #endif
    else
        return 0;
}

#elif defined(_WIN32)
#include <windows.h>
#include <psapi.h>

long getMemoryUsage() {
    PROCESS_MEMORY_COUNTERS info;
    GetProcessMemoryInfo( GetCurrentProcess(), &info, sizeof(info) );
    return (long) info.PeakWorkingSetSize; // bytes
}

#else

#warning "Rust crate `app-memory-usage-fetcher` doesn't support this operating system. Maybe file an issue at https://github.com/rjzak/app-memory-usage-fetcher/issues?"

long getMemoryUsage() {
    return -1;
}

#endif