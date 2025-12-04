cmake_minimum_required(VERSION 3.5)
project(protokit)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

set(protobuf_ABSL_PROVIDER "module" CACHE STRING "Change absl" FORCE)
set(protobuf_VERBOSE ON CACHE BOOL "enable verbose mode" FORCE)
set(protobuf_BUILD_CONFORMANCE ON CACHE BOOL "enable conformance" FORCE)
set(protobuf_BUILD_LIBPROTOC ON CACHE BOOL "enable libprotoc" FORCE)
set(protobuf_BUILD_PROTOC_BINARIES ON CACHE BOOL "Enable protoc" FORCE)
set(protobuf_SOURCE_DIR "vendor/protobuf" CACHE BOOL "vendor/protobuf" FORCE)


add_subdirectory(vendor/protobuf)

include(FetchContent)

FetchContent_Declare(
        Corrosion
        GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
        GIT_TAG v0.6 # Optionally specify a commit hash, version tag or branch here
)

FetchContent_MakeAvailable(Corrosion)

# Import targets defined in a package or workspace manifest `Cargo.toml` file
corrosion_import_crate(
        MANIFEST_PATH ${CMAKE_CURRENT_SOURCE_DIR}/tools/conformance/Cargo.toml
        CRATES conformance
)

# These files are generated as part of the conformance_cpp build
set_source_files_properties(
        ${protobuf_SOURCE_DIR}/conformance/conformance.pb.h
        ${protobuf_SOURCE_DIR}/conformance/conformance.pb.cc

        ${protobuf_SOURCE_DIR}/src/google/protobuf/test_messages_proto3.pb.h
        ${protobuf_SOURCE_DIR}/src/google/protobuf/test_messages_proto3.pb.cc
        ${protobuf_SOURCE_DIR}/src/google/protobuf/test_messages_proto2.pb.h
        ${protobuf_SOURCE_DIR}/src/google/protobuf/test_messages_proto2.pb.cc


        PROPERTIES GENERATED TRUE
)

add_executable(rust_conformance_runner
        tools/conformance/runner.cc
        ${protobuf_SOURCE_DIR}/conformance/conformance_test.cc

        ${protobuf_SOURCE_DIR}/conformance/binary_json_conformance_suite.cc
        ${protobuf_SOURCE_DIR}/conformance/binary_json_conformance_suite.h
        ${protobuf_SOURCE_DIR}/conformance/text_format_conformance_suite.cc
        ${protobuf_SOURCE_DIR}/conformance/text_format_conformance_suite.h

        ${protobuf_SOURCE_DIR}/conformance/conformance.pb.h
        ${protobuf_SOURCE_DIR}/conformance/conformance.pb.cc
        ${protobuf_SOURCE_DIR}/src/google/protobuf/test_messages_proto3.pb.h
        ${protobuf_SOURCE_DIR}/src/google/protobuf/test_messages_proto3.pb.cc
        ${protobuf_SOURCE_DIR}/src/google/protobuf/test_messages_proto2.pb.h
        ${protobuf_SOURCE_DIR}/src/google/protobuf/test_messages_proto2.pb.cc

        )

# This ensures that cmake actually generates mentioned files, raw dep doesn't work
add_dependencies(rust_conformance_runner conformance_cpp)

target_include_directories(rust_conformance_runner PUBLIC
        vendor/protobuf
        vendor/protobuf/src
        vendor/protobuf/conformance
        vendor/protobuf/third_party/abseil-cpp
        vendor/protobuf/third_party/jsoncpp/include
        )

target_link_libraries(rust_conformance_runner
        PUBLIC
        libprotobuf
        ${protobuf_ABSL_USED_TARGETS}
        jsoncpp_lib conformance
        )