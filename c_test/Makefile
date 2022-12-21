LIB_PATH ?= ../target/debug/

.PHONY: test

test: $(LIB_PATH)/libthai.so.0 test_thbrk test_thwchar test_thwbrk
	LD_LIBRARY_PATH=$(LIB_PATH) ./test_thbrk
	LD_LIBRARY_PATH=$(LIB_PATH) ./test_thwbrk
	LD_LIBRARY_PATH=$(LIB_PATH) ./test_thwchar

%: %.c
	gcc -lthai -o $@ $<

$(LIB_PATH)/libthai.so.0:
	ln -s libthai.so $(LIB_PATH)/libthai.so.0
