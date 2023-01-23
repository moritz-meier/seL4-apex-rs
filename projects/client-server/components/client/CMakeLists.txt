cmake_minimum_required(VERSION 3.18.4)
project(hello-rust C)

include(rust)

RustAddLibrary(
    client
    SOURCE_DIR
    ${CMAKE_CURRENT_LIST_DIR}
    TARGET
	armv7-unknown-linux-musleabi
    LIB_FILENAME
    libclient.a
)

DeclareCAmkESComponent(Client LIBS client)
