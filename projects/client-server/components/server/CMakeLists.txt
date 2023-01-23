cmake_minimum_required(VERSION 3.18.4)
project(hello-rust C)

include(rust)

RustAddLibrary(
    server
    SOURCE_DIR
    ${CMAKE_CURRENT_LIST_DIR}
    TARGET
	armv7-unknown-linux-musleabi
    LIB_FILENAME
    libserver.a
)

DeclareCAmkESComponent(Server LIBS server)
