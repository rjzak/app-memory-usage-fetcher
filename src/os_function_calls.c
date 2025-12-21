// SPDX-License-Identifier: Apache-2.0

#if defined(__APPLE__) || defined(__linux__) || defined(__FreeBSD__) || defined(__OpenBSD__) || defined(__NetBSD__) || defined(__DragonFly__) || defined(_AIX) || defined(__ANDROID__)
#include <sys/resource.h>

// macOS: bytes https://unix.stackexchange.com/q/30940
// BSD: kilobytes http://stackoverflow.com/q/40773924/781723, https://man.openbsd.org/getrusage.2, https://man.netbsd.org/getrusage.2
// Linux: kilobytes http://stackoverflow.com/q/12050913/781723, https://www.gnu.org/software/libc/manual/html_node/Resource-Usage.html
// AIX: kilobytes https://www.ibm.com/docs/en/aix/7.3.0#getrusage_64__row-d3e114410
// Solaris: zero, doesn't return memory usage https://docs.oracle.com/cd/E36784_01/html/E36874/getrusage-3c.html

long getMemoryUsage() {
    struct rusage usage;
    if(0 == getrusage(RUSAGE_SELF, &usage))
        #if defined(__APPLE__)
            return usage.ru_maxrss; // bytes
        #else
            return usage.ru_maxrss * 1024; // kilobytes to bytes
        #endif
    else
        return 0;
}

#elif defined(_WIN32)
#include <windows.h>
#include <psapi.h>

// Windows: bytes https://learn.microsoft.com/en-us/windows/win32/api/psapi/ns-psapi-process_memory_counters

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