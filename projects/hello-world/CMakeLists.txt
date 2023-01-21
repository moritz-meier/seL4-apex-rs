cmake_minimum_required(VERSION 3.18.4)

project(hello-world C ASM)

DeclareCAmkESComponent(Hello SOURCES hello.c)
DeclareCAmkESRootserver(hello.camkes)
