#CC=g++
TIMEFLAGS=-f "%e"

all: cxx rust swift

cxx: main.cpp
	$(CXX) -O3 -std=c++14 main.cpp -o main_cxx
rust: main.rs
	rustc main.rs -O -o main_rs
swift: main.swift
	swiftc main.swift -O -o main_swift

clean: 
	rm -f main_*

bench: all
	time $(TIMEFLAGS) ./main_cxx
	time $(TIMEFLAGS) ./main_rs
	time $(TIMEFLAGS) ./main_swift
