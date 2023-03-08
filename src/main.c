#include <stdio.h>
#include <pthread.h>
#include "sub.h"

pthread_mutex_t lock;

void *thread_func(void *arg) {
    //
    pthread_mutex_lock(&lock);
    int result = sub1();
    pthread_mutex_unlock(&lock);
    return (void*)result;
}

int main(void) {
    pthread_t thread1, thread2;
    int result1, result2;

    //
    pthread_mutex_init(&lock, NULL);

    //
    pthread_create(&thread1, NULL, thread_func, NULL);

    //
    pthread_create(&thread2, NULL, thread_func, NULL);

    //
    pthread_join(thread1, (void **)&result1);
    pthread_join(thread2, (void **)&result2);

    //
    printf("Result 1: %d\n", result1);
    printf("Result 2: %d\n", result2);

    //
    pthread_mutex_destroy(&lock);

    return 0;
}
