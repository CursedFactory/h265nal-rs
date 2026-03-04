cmake_minimum_required(VERSION 3.10)

project(h265nal VERSION 0.53)

set(CMAKE_CXX_STANDARD 14)

if(WIN32)
  set(CMAKE_CXX_STANDARD 20)
  if(CMAKE_CXX_COMPILER_ID MATCHES "MSVC")
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} /std:c++20")
  else()
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++20")
  endif()
else()
  set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++14")
endif()

set(COMMON_DEBUG_FLAGS "-g -O0 -Wall -Wextra -Wunused-parameter -Wshadow -Wformat -Wextra-semi -Wsign-conversion -Werror")
if(CMAKE_CXX_COMPILER_ID MATCHES "Clang")
  set(COMMON_DEBUG_FLAGS "${COMMON_DEBUG_FLAGS} -Wextra-semi-stmt")
endif()
set(CMAKE_C_FLAGS_DEBUG "${COMMON_DEBUG_FLAGS}")
set(CMAKE_CXX_FLAGS_DEBUG "${COMMON_DEBUG_FLAGS}")

add_subdirectory(src)
