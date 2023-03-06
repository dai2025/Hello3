CC=gcc
CFLAGS=-O0 -Wall -std=c11
OBJS=Hello.o Hello3.o

TARGET=Hello3

# gcc -pass-exit-codes
# spaces -> Tabs 8
$(TARGET): $(OBJS)
	$(CC) -o $@ $(OBJS)
clean:
	rm -f *.o Hello3
check:
	./$(TARGET)
