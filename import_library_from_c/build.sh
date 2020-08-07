#!/bin/bash

CROSSCOMPILER=
IS_MINGW_W64=

build_lib() {
	${CROSSCOMPILER}gcc mylib.c -O3 -c -fPIC -o mylib.o
	if [ -z "$IS_MINGW_W64" ]; then
		echo 1
		${CROSSCOMPILER}g++ -shared -o libmylib.so mylib.o
	else
		echo 2
		${CROSSCOMPILER}g++ -shared -o libmylib.dll \
			-Wl,--out-implib=libmylib.dll.a \
			-Wl,--export-all-symbols \
			-Wl,--enable-auto-import \
			-Wl,--whole-archive mylib.o \
			-Wl,--no-whole-archive -lgdi32 -luser32 -lkernel32
	fi
}

cross_build_mingw_w64() {
	IS_MINGW_W64=1
	CROSSCOMPILER=x86_64-w64-mingw32-
	build_lib
	cargo build --target x86_64-pc-windows-gnu
}

build_host() {
	build_lib
	cargo build
}

build() {
	build_host
	cross_build_mingw_w64
}

run_host() {
	LD_LIBRARY_PATH=`pwd` ./target/debug/import_library_from_c
}

clean() {
	rm -rf main target mylib.o libmylib.so libmylib.dll libmylib.dll.a
}

$@
