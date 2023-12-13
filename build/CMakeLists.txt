project(ray_tracer)   
cmake_minimum_required(VERSION 3.1.1)
add_subdirectory(${CMAKE_CURRENT_SOURCE_DIR}/../src/include
    ${CMAKE_CURRENT_BINARY_DIR}/bin_directories)
add_executable(main ../src/main.c++)
target_link_libraries(main ppm colour vec3)
set_target_properties(main 
    PROPERTIES
    ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
    LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
    RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin"
)
