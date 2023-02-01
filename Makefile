CFLAGS=-std=c11 -g -static

llcc: llcc.c

test: llcc
	./test.sh

clean:
	rm -f llcc *.o *~ tmp*

.PHONY: test clean
