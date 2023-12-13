project("Ray-Tracer")   
cmake_minimum_required(VERSION 3.1.1)
add_subdirectory(${CMAKE_CURRENT_SOURCE_DIR}/../src/include ${CMAKE_CURRENT_BINARY_DIR}/bin_directories)
add_executable(output ../src/main.c++)
target_link_libraries(output display)
set_target_properties(output
    PROPERTIES
    ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
    LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
    RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin"
)
