find_package(Catch2 3 REQUIRED)
add_subdirectory(${CMAKE_CURRENT_SOURCE_DIR}/../../src/include
    ${CMAKE_CURRENT_BINARY_DIR}/bin_directories)
# These tests can use the Catch2-provided main
add_executable(tests ../tests.c++)
target_link_libraries(tests PRIVATE Catch2::Catch2WithMain vec3)

# These tests need their own main
#add_executable(custom-main-tests test.cpp test-main.cpp)
#target_link_libraries(custom-main-tests PRIVATE Catch2::Catch2)
# Include to have the test suite built with compilation
# add_subdirectory(${CMAKE_CURRENT_SOURCE_DIR}/../tests
#    ${CMAKE_CURRENT_BINARY_DIR}/tests)
set_target_properties(tests
    PROPERTIES
    ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
    LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
    RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin"
)
