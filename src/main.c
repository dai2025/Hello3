#include <stdio.h>
#include <pthread.h>
#include "sub.h"

void *thread_function(void *arg) {
  printf("Thread running\n");
  int result = sub();
  printf("Result from thread: %d\n", result);
  pthread_exit(NULL);
}

int main() {
  pthread_t my_thread;
  printf("Creating thread\n");
  if (pthread_create(&my_thread, NULL, thread_function, NULL)) {
    printf("error creating thread.");
    abort();
  }
  printf("Waiting for thread to finish\n");
  if (pthread_join(my_thread, NULL)) {
    printf("error joining thread.");
    abort();
  }
  printf("Thread finished\n");
  return 0;
}
