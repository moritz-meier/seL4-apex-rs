cmake_minimum_required(VERSION 3.18.4)
project(hello-rust C)

include(rust)

RustAddLibrary(
    main
    SOURCE_DIR
    ${CMAKE_CURRENT_LIST_DIR}
    TARGET
	armv7-unknown-linux-musleabi
    LIB_FILENAME
    libmain.a
)

DeclareCAmkESComponent(main LIBS main)
DeclareCAmkESRootserver(hello-rust.camkes)
