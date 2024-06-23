cmake_minimum_required(VERSION 3.10)
project(RustProject NONE)  # The NONE argument indicates that this project does not follow the typical C++/C project structure

# Custom target for building the Rust project
add_custom_target(build ALL
    COMMAND cargo build
    COMMAND ${CMAKE_COMMAND} -E copy_if_different ${CMAKE_CURRENT_SOURCE_DIR}/target/debug/rustmove ${CMAKE_CURRENT_BINARY_DIR}/rustmove
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    COMMENT "Building Rust project"
)

# Custom target for testing the Rust project
add_custom_target(test
    COMMAND cargo test
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    COMMENT "Testing Rust project"
)

