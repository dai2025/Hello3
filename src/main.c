#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>

//
extern int sub(void);

//
void* thread_function(void* arg) {
    printf("Thread starting...\n");
    int result = sub();
    printf("Sub function result: %d\n", result);
    printf("Thread exiting...\n");
    return NULL;
}

int main(int argc, char* argv[]) {
    //
    pthread_t thread;
    if (pthread_create(&thread, NULL, thread_function, NULL)) {
        fprintf(stderr, "Error creating thread\n");
        return 1;
    }

    printf("Main function executing...\n");
    //
    if (pthread_join(thread, NULL)) {
        fprintf(stderr, "Error joining thread\n");
        return 2;
    }

    printf("Main function exiting...\n");
    return 0;
}
