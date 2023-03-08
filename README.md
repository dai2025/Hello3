# Hello3

### Multithreaded programming is a difficult topic for beginners. When programming with multithreading, it is important to keep in mind the following points to ensure safety:

1. Write thread-safe code:
In a multithreaded program, multiple threads are executed at the same time. Therefore, there is a possibility that multiple threads access the same data. Therefore, it is important to write thread-safe code.

2. Avoid deadlocks:
If multiple threads depend on each other, a deadlock may occur. A deadlock is a phenomenon where two or more threads are waiting for each other, making it impossible to proceed. To avoid this, it is necessary to unify the order in which locks are acquired.

3. Avoid race conditions:
When multiple threads change the same variable or resource at the same time, a race condition may occur. This is a phenomenon where the result becomes unpredictable. To avoid this, you need to use exclusion control or synchronization mechanism to prevent threads from interfering with each other.

4. Manage threads:
Once you have created threads, it is important to manage them properly. You need to start and end threads properly to ensure that they terminate normally. Also, if a thread enters an infinite loop, the program may freeze, so you need to control it properly.

When programming with multithreading, it is important to keep these points in mind to ensure safety. Beginners should be especially careful to use exclusion control or synchronization mechanism appropriately to prevent threads from interfering with each other.
