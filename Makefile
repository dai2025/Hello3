CC=gcc
CFLAGS=-O0 -Wall
OBJS=Hello.o Hello3.o

TARGET=Hello3

#speces
#spaces -> Tabs 8
$(TARGET): $(OBJS)
	$(CC) -o $@ $(OBJS)
clean:
	rm -f *.o Hello3
