// SPDX-License-Identifier: Apache-2.0

#if defined(__APPLE__) || defined(__linux__)

#include <sys/time.h>
#include <sys/resource.h>

long getMemoryUsage() {
    struct rusage usage;
    if(0 == getrusage(RUSAGE_SELF, &usage))
        return usage.ru_maxrss; // bytes
    else
        return 0;
}

#elif defined(_WIN32)
#include <windows.h>
#include <psapi.h>

long getMemoryUsage() {
    PROCESS_MEMORY_COUNTERS info;
    GetProcessMemoryInfo( GetCurrentProcess(), &info, sizeof(info) );
    return (long) info.PeakWorkingSetSize;
}

#else

long getMemoryUsage() {
    return -1;
}


#endif