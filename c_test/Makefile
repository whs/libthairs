LIB_PATH ?= ../target/debug/libthai.so

.PHONY: test

test: test_thbrk
	LD_PRELOAD=$(LIB_PATH) ./test_thbrk

test_thbrk: test_thbrk.c
	gcc -lthai -o $@ $<
