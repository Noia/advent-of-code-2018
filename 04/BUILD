load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary", "rust_library")

rust_library(
  name="guards",
  srcs=["guards.rs"],
  deps=[
    "//third_party/cargo:scan_fmt"
  ]
)
rust_binary(
  name="part1",
  srcs=["part1.rs"],
  data=["input.txt"],
  deps=[
    ":guards",
    "//third_party/cargo:scan_fmt"
  ]
)

rust_binary(
  name="hello",
  srcs=["hello.rs"],
)
