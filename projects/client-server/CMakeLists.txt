cmake_minimum_required(VERSION 3.18.4)
project(hello-rust C)

include("${CMAKE_CURRENT_LIST_DIR}/components/client/CMakeLists.txt")

DeclareCAmkESRootserver(client-server.camkes)
