# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 3.22

# Delete rule output on recipe failure.
.DELETE_ON_ERROR:

#=============================================================================
# Special targets provided by cmake.

# Disable implicit rules so canonical targets will work.
.SUFFIXES:

# Disable VCS-based implicit rules.
% : %,v

# Disable VCS-based implicit rules.
% : RCS/%

# Disable VCS-based implicit rules.
% : RCS/%,v

# Disable VCS-based implicit rules.
% : SCCS/s.%

# Disable VCS-based implicit rules.
% : s.%

.SUFFIXES: .hpux_make_needs_suffix_list

# Command-line flag to silence nested $(MAKE).
$(VERBOSE)MAKESILENT = -s

#Suppress display of executed commands.
$(VERBOSE).SILENT:

# A target that is always out of date.
cmake_force:
.PHONY : cmake_force

#=============================================================================
# Set environment variables for the build.

# The shell in which to execute make rules.
SHELL = /bin/sh

# The CMake executable.
CMAKE_COMMAND = /usr/bin/cmake

# The command to remove a file.
RM = /usr/bin/cmake -E rm -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = /home/js/c++/ray-tracer/build

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = /home/js/c++/ray-tracer/build

# Include any dependencies generated for this target.
include bin_directories/colour/CMakeFiles/colour.dir/depend.make
# Include any dependencies generated by the compiler for this target.
include bin_directories/colour/CMakeFiles/colour.dir/compiler_depend.make

# Include the progress variables for this target.
include bin_directories/colour/CMakeFiles/colour.dir/progress.make

# Include the compile flags for this target's objects.
include bin_directories/colour/CMakeFiles/colour.dir/flags.make

bin_directories/colour/CMakeFiles/colour.dir/colour.c++.o: bin_directories/colour/CMakeFiles/colour.dir/flags.make
bin_directories/colour/CMakeFiles/colour.dir/colour.c++.o: /home/js/c++/ray-tracer/src/include/colour/colour.c++
bin_directories/colour/CMakeFiles/colour.dir/colour.c++.o: bin_directories/colour/CMakeFiles/colour.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/home/js/c++/ray-tracer/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_1) "Building CXX object bin_directories/colour/CMakeFiles/colour.dir/colour.c++.o"
	cd /home/js/c++/ray-tracer/build/bin_directories/colour && /usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT bin_directories/colour/CMakeFiles/colour.dir/colour.c++.o -MF CMakeFiles/colour.dir/colour.c++.o.d -o CMakeFiles/colour.dir/colour.c++.o -c /home/js/c++/ray-tracer/src/include/colour/colour.c++

bin_directories/colour/CMakeFiles/colour.dir/colour.c++.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing CXX source to CMakeFiles/colour.dir/colour.c++.i"
	cd /home/js/c++/ray-tracer/build/bin_directories/colour && /usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /home/js/c++/ray-tracer/src/include/colour/colour.c++ > CMakeFiles/colour.dir/colour.c++.i

bin_directories/colour/CMakeFiles/colour.dir/colour.c++.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling CXX source to assembly CMakeFiles/colour.dir/colour.c++.s"
	cd /home/js/c++/ray-tracer/build/bin_directories/colour && /usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /home/js/c++/ray-tracer/src/include/colour/colour.c++ -o CMakeFiles/colour.dir/colour.c++.s

bin_directories/colour/CMakeFiles/colour.dir/home/js/c++/ray-tracer/src/include/vec3/vec3.c++.o: bin_directories/colour/CMakeFiles/colour.dir/flags.make
bin_directories/colour/CMakeFiles/colour.dir/home/js/c++/ray-tracer/src/include/vec3/vec3.c++.o: /home/js/c++/ray-tracer/src/include/vec3/vec3.c++
bin_directories/colour/CMakeFiles/colour.dir/home/js/c++/ray-tracer/src/include/vec3/vec3.c++.o: bin_directories/colour/CMakeFiles/colour.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --progress-dir=/home/js/c++/ray-tracer/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_2) "Building CXX object bin_directories/colour/CMakeFiles/colour.dir/home/js/c++/ray-tracer/src/include/vec3/vec3.c++.o"
	cd /home/js/c++/ray-tracer/build/bin_directories/colour && /usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -MD -MT bin_directories/colour/CMakeFiles/colour.dir/home/js/c++/ray-tracer/src/include/vec3/vec3.c++.o -MF CMakeFiles/colour.dir/home/js/c++/ray-tracer/src/include/vec3/vec3.c++.o.d -o CMakeFiles/colour.dir/home/js/c++/ray-tracer/src/include/vec3/vec3.c++.o -c /home/js/c++/ray-tracer/src/include/vec3/vec3.c++

bin_directories/colour/CMakeFiles/colour.dir/home/js/c++/ray-tracer/src/include/vec3/vec3.c++.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Preprocessing CXX source to CMakeFiles/colour.dir/home/js/c++/ray-tracer/src/include/vec3/vec3.c++.i"
	cd /home/js/c++/ray-tracer/build/bin_directories/colour && /usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -E /home/js/c++/ray-tracer/src/include/vec3/vec3.c++ > CMakeFiles/colour.dir/home/js/c++/ray-tracer/src/include/vec3/vec3.c++.i

bin_directories/colour/CMakeFiles/colour.dir/home/js/c++/ray-tracer/src/include/vec3/vec3.c++.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green "Compiling CXX source to assembly CMakeFiles/colour.dir/home/js/c++/ray-tracer/src/include/vec3/vec3.c++.s"
	cd /home/js/c++/ray-tracer/build/bin_directories/colour && /usr/bin/c++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -S /home/js/c++/ray-tracer/src/include/vec3/vec3.c++ -o CMakeFiles/colour.dir/home/js/c++/ray-tracer/src/include/vec3/vec3.c++.s

# Object files for target colour
colour_OBJECTS = \
"CMakeFiles/colour.dir/colour.c++.o" \
"CMakeFiles/colour.dir/home/js/c++/ray-tracer/src/include/vec3/vec3.c++.o"

# External object files for target colour
colour_EXTERNAL_OBJECTS =

bin_directories/colour/libcolour.a: bin_directories/colour/CMakeFiles/colour.dir/colour.c++.o
bin_directories/colour/libcolour.a: bin_directories/colour/CMakeFiles/colour.dir/home/js/c++/ray-tracer/src/include/vec3/vec3.c++.o
bin_directories/colour/libcolour.a: bin_directories/colour/CMakeFiles/colour.dir/build.make
bin_directories/colour/libcolour.a: bin_directories/colour/CMakeFiles/colour.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color --switch=$(COLOR) --green --bold --progress-dir=/home/js/c++/ray-tracer/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_3) "Linking CXX static library libcolour.a"
	cd /home/js/c++/ray-tracer/build/bin_directories/colour && $(CMAKE_COMMAND) -P CMakeFiles/colour.dir/cmake_clean_target.cmake
	cd /home/js/c++/ray-tracer/build/bin_directories/colour && $(CMAKE_COMMAND) -E cmake_link_script CMakeFiles/colour.dir/link.txt --verbose=$(VERBOSE)

# Rule to build all files generated by this target.
bin_directories/colour/CMakeFiles/colour.dir/build: bin_directories/colour/libcolour.a
.PHONY : bin_directories/colour/CMakeFiles/colour.dir/build

bin_directories/colour/CMakeFiles/colour.dir/clean:
	cd /home/js/c++/ray-tracer/build/bin_directories/colour && $(CMAKE_COMMAND) -P CMakeFiles/colour.dir/cmake_clean.cmake
.PHONY : bin_directories/colour/CMakeFiles/colour.dir/clean

bin_directories/colour/CMakeFiles/colour.dir/depend:
	cd /home/js/c++/ray-tracer/build && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /home/js/c++/ray-tracer/build /home/js/c++/ray-tracer/src/include/colour /home/js/c++/ray-tracer/build /home/js/c++/ray-tracer/build/bin_directories/colour /home/js/c++/ray-tracer/build/bin_directories/colour/CMakeFiles/colour.dir/DependInfo.cmake --color=$(COLOR)
.PHONY : bin_directories/colour/CMakeFiles/colour.dir/depend

