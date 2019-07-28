CC = gcc
OBJECTS = libsvm_wasmer_c_api.a libsvm_wasmer_c_api.dylib
CFLAGS = -g -Wall -I
DEPS = svm_wasmer.h

default: counter

%.o: %.c $(DEPS)
	$(CC) -c -o $@ $< $(CFLAGS)

counter: $(DEPS)
	$(CC) counter.c $(OBJECTS) -o counter

clean:
	-rm -f $(OBJECTS)
	-rm -f counter
