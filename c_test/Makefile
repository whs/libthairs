LIB_PATH ?= ../target/debug/libthai.so

.PHONY: test

test: test_thbrk test_thwchar
	LD_PRELOAD=$(LIB_PATH) ./test_thbrk
	LD_PRELOAD=$(LIB_PATH) ./test_thwchar

test_thbrk: test_thbrk.c
	gcc -lthai -o $@ $<

test_thwchar: test_thwchar.c
	gcc -lthai -o $@ $<
