load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary")

rust_binary(
  name="part1",
  srcs=["part1.rs"],
  data=["input.txt"],
  deps=[
    "//third_party/cargo:scan_fmt"
  ]
)
