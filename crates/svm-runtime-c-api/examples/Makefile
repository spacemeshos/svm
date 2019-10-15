CC = gcc
CFLAGS = -g -Wall -I
DEPS = counter.c wasmer.h svm_wasmer.h

UNAME := $(shell uname)

ifeq ($(UNAME), Darwin)
	OBJECTS = libwasmer_runtime_c_api.dylib libsvm_wasmer_c_api.dylib
endif
ifeq ($(UNAME), Linux)
	OBJECTS = libwasmer_runtime_c_api.so libsvm_wasmer_c_api.so
endif

default: counter

%.o: %.c $(DEPS)
	$(CC) -c -o $@ $< $(CFLAGS)

counter: $(DEPS)
	$(CC) $(OBJECTS) counter.c -o counter.out

clean:
	-rm -f $(OBJECTS)
	-rm -f counter
