LIB_PATH ?= ../target/debug/libthai.so

.PHONY: test

test: test_thbrk test_thwchar test_thwbrk
	LD_PRELOAD=$(LIB_PATH) ./test_thbrk
	LD_PRELOAD=$(LIB_PATH) ./test_thwbrk
	LD_PRELOAD=$(LIB_PATH) ./test_thwchar

%: %.c
	gcc -lthai -o $@ $<
