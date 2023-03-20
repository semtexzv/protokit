cmake_minimum_required(VERSION 3.5)
project(protokit)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

set(protobuf_BUILD_CONFORMANCE ON CACHE BOOL "enable conformance" FORCE)
set(protobuf_BUILD_LIBPROTOC ON CACHE BOOL "enable libprotoc" FORCE)

add_subdirectory(vendor/protobuf)

include(FetchContent)

FetchContent_Declare(
        Corrosion
        GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
        GIT_TAG v0.3 # Optionally specify a commit hash, version tag or branch here
)
FetchContent_MakeAvailable(Corrosion)

# Import targets defined in a package or workspace manifest `Cargo.toml` file
corrosion_import_crate(
    MANIFEST_PATH ${CMAKE_CURRENT_SOURCE_DIR}/tools/conformance/Cargo.toml
    CRATES conformance
)

add_executable(rust_conformance_runner
        tools/conformance/runner.cc
        ${protobuf_SOURCE_DIR}/conformance/binary_json_conformance_suite.cc
        ${protobuf_SOURCE_DIR}/conformance/binary_json_conformance_suite.h
        ${protobuf_SOURCE_DIR}/conformance/conformance.pb.h
        ${protobuf_SOURCE_DIR}/conformance/conformance.pb.cc
        ${protobuf_SOURCE_DIR}/conformance/conformance_test.cc
        ${protobuf_SOURCE_DIR}/conformance/conformance_test_runner.cc
        ${protobuf_SOURCE_DIR}/conformance/text_format_conformance_suite.cc
        ${protobuf_SOURCE_DIR}/conformance/text_format_conformance_suite.h
        ${protobuf_SOURCE_DIR}/src/google/protobuf/test_messages_proto2.pb.h
        ${protobuf_SOURCE_DIR}/src/google/protobuf/test_messages_proto2.pb.cc
        ${protobuf_SOURCE_DIR}/src/google/protobuf/test_messages_proto3.pb.h
        ${protobuf_SOURCE_DIR}/src/google/protobuf/test_messages_proto3.pb.cc
        )

target_include_directories(rust_conformance_runner PUBLIC
        vendor/protobuf
        vendor/protobuf/src
        vendor/protobuf/conformance
        vendor/protobuf/third_party/abseil-cpp
        vendor/protobuf/third_party/jsoncpp/include
        )

target_link_libraries(rust_conformance_runner PUBLIC libprotobuf ${protobuf_ABSL_USED_TARGETS} jsoncpp_lib conformance)