TIMEFLAGS=-f "%e"
OUT=2>&1 1>/dev/null

all: clang gcc rust swift

gcc: main.cpp
	g++ -O3 -std=c++14 ./main.cpp -o ./main_gcc
clang: main.cpp
	clang++ -O3 -std=c++14 ./main.cpp -o ./main_clang
rust: ./main.rs
	rustc ./main.rs -O -o ./main_rs
swift: main.swift
	swiftc ./main.swift -O -o ./main_swift

clean:
	rm -f main_*

bench: all
	time $(TIMEFLAGS) ./main_gcc $(OUT)
	time $(TIMEFLAGS) ./main_clang $(OUT)
	time $(TIMEFLAGS) ./main_rs $(OUT)
	time $(TIMEFLAGS) ./main_swift $(OUT)
