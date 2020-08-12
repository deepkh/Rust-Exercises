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

_build() {
	build_lib
	cargo build --target ${TARGET_PLATFORM} ${RELEASE}
}

_build_mingw_w64() {
	IS_MINGW_W64=1
	CROSSCOMPILER=x86_64-w64-mingw32-
	TARGET_PLATFORM="x86_64-pc-windows-gnu"
	_build
}

dev_mingw_w64() {
	_build_mingw_w64
}

release_mingw_w64() {
	RELEASE="--release"
	_build_mingw_w64
}

_build_linux_x86_64() {
	TARGET_PLATFORM="x86_64-unknown-linux-gnu"
	_build
}

dev() {
	_build_linux_x86_64
}

release() {
	RELEASE="--release"
	_build_linux_x86_64
}

run() {
	LD_LIBRARY_PATH=`pwd` ./target/debug/import_library_from_c
}

run_mingw_w64() {
	LD_LIBRARY_PATH=`pwd` ./target/x86_64-pc-windows-gnu/debug/import_library_from_c.exe
}

clean() {
	rm -rf main target mylib.o libmylib.so libmylib.dll libmylib.dll.a
}

$@
