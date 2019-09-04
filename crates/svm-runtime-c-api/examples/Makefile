CC = gcc
OBJECTS = libwasmer_runtime_c_api.dylib libsvm_wasmer_c_api.dylib
CFLAGS = -g -Wall -I
DEPS = counter.c wasmer.h svm_wasmer.h

default: counter

%.o: %.c $(DEPS)
	$(CC) -c -o $@ $< $(CFLAGS)

counter: $(DEPS)
	$(CC) $(OBJECTS) counter.c -o counter.out

clean:
	-rm -f $(OBJECTS)
	-rm -f counter
