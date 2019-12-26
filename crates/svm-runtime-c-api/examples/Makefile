CC = gcc
OBJECTS = libwasmer_runtime_c_api.dylib svm.dylib
CFLAGS = -g -Wall -I
DEPS = counter.c wasmer.h svm.h

default: counter

%.o: %.c $(DEPS)
	$(CC) -c -o $@ $< $(CFLAGS)

counter: $(DEPS)
	$(CC) $(CFLAGS) $(OBJECTS) counter.c -o counter.out

clean:
	-rm svm
	-rm counter.out
