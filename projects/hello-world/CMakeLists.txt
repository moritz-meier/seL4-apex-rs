cmake_minimum_required(VERSION 3.18.4)
project(hello-world C)

DeclareCAmkESComponent(HelloWorld SOURCES hello-world.c)
DeclareCAmkESRootserver(hello-world.camkes)
