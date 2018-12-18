load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary", "rust_library")

rust_library(
  name="claim",
  srcs=["claim.rs"],
  deps=[
    "//third_party/cargo:scan_fmt"
  ]
)

rust_binary(
  name="part1",
  srcs=["part1.rs"],
  data=["input.txt"],
  deps=[":claim"]
)

rust_binary(
  name="part2",
  srcs=["part2.rs"],
  data=["input.txt"],
  deps=[
    ":claim",
    "//third_party/cargo:scan_fmt"
  ]
)
