CC = gcc
OBJECTS = ../svm.dylib
CFLAGS = -g -Wall
DEPS = counter.c ../svm.h

default: counter

%.o: %.c $(DEPS)
	$(CC) -c -o $@ $< $(CFLAGS)

counter: $(DEPS)
	$(CC) $(CFLAGS) $(OBJECTS) counter.c -o counter.out

clean:
	-rm svm
	-rm counter.out
