#!/bin/bash

list_exports() {
	nm -D target/debug/libexport_rust_library_to_c.so  | grep ' T'
	#x86_64-w64-mingw32-nm target/x86_64-pc-windows-gnu/debug/export_rust_library_to_c.dll
}

build_rust_lib() {
	cargo build
	# cross build: target to windows
	#cargo build --target x86_64-pc-windows-gnu
}

build() {
	build_rust_lib
	gcc -g -o main main.c -lexport_rust_library_to_c -L./target/debug
}

run() {
	LD_LIBRARY_PATH=./target/debug/ ./main
}

clean() {
	rm -rf main target
}

$@
