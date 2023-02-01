CFLAGS=-std=c11 -g -static
SRCS=$(wildcard *.c)
OBJS=$(SRCS:.c=.o)

llcc: $(OBJS)
	$(CC) -o llcc $(OBJS) $(LDFLAGS)

$(OBJS): llcc.h

test: llcc
	./test.sh

clean:
	rm -f llcc *.o *~ tmp*

.PHONY: test clean
