load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary", "rust_library")

rust_library(
  name="claim",
  srcs=["claim.rs"],
  deps=[
    "//third_party/cargo:scan_fmt"
  ]
)

rust_binary(
  name="day3_part1",
  srcs=["day3_part1.rs"],
  data=["input.txt"],
  deps=[":claim"]
)

rust_binary(
  name="day3_part2",
  srcs=["day3_part2.rs"],
  data=["input.txt"],
  deps=[
    ":claim",
    "//third_party/cargo:scan_fmt"
  ]
)
