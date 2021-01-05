#include "spinlock.h"
#include <kernel/multitasking/tasks/task_small.h>

static inline bool atomic_compare_exchange(int* ptr, int compare, int exchange) {
    return __atomic_compare_exchange_n(ptr, &compare, exchange,
            false, __ATOMIC_SEQ_CST, __ATOMIC_SEQ_CST);
}

static inline void atomic_store(int* ptr, int value) {
    __atomic_store_n(ptr, value, __ATOMIC_SEQ_CST);
}

static inline int atomic_add_fetch(int* ptr, int d) {
    return __atomic_add_fetch(ptr, d, __ATOMIC_SEQ_CST);
}

void spinlock_acquire(spinlock_t* lock) {
    if (!lock) return;
    assert(lock->name, "Spinlock was used without assigning a name");

    // Keep track of whether we had to wait to acquire the lock
	uint32_t contention_start = 0;

	if (lock->flag != 0) {
        // Check for data corruption
		assert(lock->flag == 1, "Bad lock flag");

        // We should never be waiting on a spinlock while interrupts are disabled
        // (This means another task has acquired the spinlock, 
        // and since interrupts are disabled we will never context switch back to the 
        // other task)
        if (!interrupts_enabled()) { 
            printf("Spinlock %s held by another consumer while interrupts are disabled\n", lock->name); 
            printf("Proc [%d] holds the spinlock!\n", lock->holder_pid);
        }
        assert(interrupts_enabled(), "Spinlock held by another consumer while interrupts are disabled");

		contention_start = time();

		printf("Spinlock: [%d] found contended spinlock with flag %d at %d\n", getpid(), lock->flag, contention_start);
	}

    // Spin until the lock is released
    while (!atomic_compare_exchange(&lock->flag, 0, 1)) {
		asm("pause");
    }

    // Prevent another context on this processor from acquiring the spinlock
    // atomic_store(&lock->interrupts_enabled_before_acquire, interrupts_enabled());
    lock->interrupts_enabled_before_acquire = interrupts_enabled();
    if (lock->interrupts_enabled_before_acquire) {
        asm("cli");
    }

    // Ensure it's really ours
    assert(lock->flag == 1, "Lock was not properly acquired");

	if (contention_start) {
		printf("Spinlock: *** Proc %d received contended lock 0x%08x %s after %d ticks\n", getpid(), lock, lock->name, time() - contention_start);
	}
    else {
        //printf("Spinlock: Proc %d received uncontended lock 0x%08x %s\n", getpid(), lock, lock->name);
    }
}

void spinlock_release(spinlock_t* lock) {
    if (!lock) return;
    atomic_store(&lock->flag, 0);
    // Allow other contexts on this processor to interact with the spinlock
    if (lock->interrupts_enabled_before_acquire) {
        asm("sti");
    }
    //printf("Spinlock: Proc %d freed lock 0x%08x %s\n", getpid(), lock, lock->name);
}