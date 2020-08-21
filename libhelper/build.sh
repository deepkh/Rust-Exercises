#!/bin/bash

#############################################################
# Build: 
#     Host from linux_x86_64 and target for linux_x86_64
#         Release   
#             ./build.sh release
#         Debug  
#             ./build.sh dev
#
#     Host from linux_x86_64 and target for windows_x86_64
#     Please install mingw-w64 cross compiler toolchain on linux_x86_64 host machine before build process
#         Release 
#            ./build.sh release_mingw_w64
#         Debug 
#            ./build.sh dev_mingw_w64
#
# Clean:
#     ./build.sh clean
#
# Run:
#     Host from linux_x86_64
#          ./build.sh run
#     Host from windows msys2
#          ./build.sh run_mingw_w64
#
#############################################################


CROSSCOMPILER=
IS_MINGW_W64=
TARGET_BIN=others
CLEAN_LIST="target"

#build_lib() {
#	${CROSSCOMPILER}gcc mylib.c -O3 -c -fPIC -o mylib.o
#	if [ -z "$IS_MINGW_W64" ]; then
#		echo ${TARGET_PLATFORM} ${RELEASE}
#		${CROSSCOMPILER}g++ -shared -o libmylib.so mylib.o
#	else
#		echo ${TARGET_PLATFORM} ${RELEASE}
#		${CROSSCOMPILER}g++ -shared -o libmylib.dll \
#			-Wl,--out-implib=libmylib.dll.a \
#			-Wl,--export-all-symbols \
#			-Wl,--enable-auto-import \
#			-Wl,--whole-archive mylib.o \
#			-Wl,--no-whole-archive -lgdi32 -luser32 -lkernel32
#	fi
#}

_build() {
#	build_lib
	cargo build --target ${TARGET_PLATFORM} ${RELEASE}
}

_build_mingw_w64() {
	IS_MINGW_W64=1
	CROSSCOMPILER=x86_64-w64-mingw32-
	TARGET_PLATFORM="x86_64-pc-windows-gnu"
	_build
}

dev_mingw_w64() {
	RELEASE=""
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
	RELEASE=""
	_build_linux_x86_64
}

release() {
	RELEASE="--release"
	_build_linux_x86_64
}

all() {
	release
	dev
	release_mingw_w64
	dev_mingw_w64
}

run() {
	#LD_LIBRARY_PATH=`pwd` ./target/debug/${TARGET_BIN}
	cargo run --bin ${TARGET_BIN}
}

run_mingw_w64() {
	LD_LIBRARY_PATH=`pwd` ./target/x86_64-pc-windows-gnu/debug/${TARGET_BIN}.exe
}

clean() {
	rm -rf ${CLEAN_LIST}
}

$@
