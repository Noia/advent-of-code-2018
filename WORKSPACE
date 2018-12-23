
load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository")
git_repository(
  name = "io_bazel_rules_rust",
  commit = "23ab9d5ed7e78ed3c9fd2cf7627eaa6ea60742bf",
  remote = "https://github.com/bazelbuild/rules_rust.git",
)

git_repository(
    name = "bazel_skylib",
    remote = "https://github.com/bazelbuild/bazel-skylib.git",
    tag = "0.6.0",
)

git_repository(
    name = "build_bazel_rules_nodejs",
    remote = "https://github.com/bazelbuild/rules_nodejs.git",
    tag = "0.16.4",
)

load("@build_bazel_rules_nodejs//:package.bzl", "rules_nodejs_dependencies")
load("@build_bazel_rules_nodejs//:defs.bzl", "node_repositories")
load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repositories")
load("@io_bazel_rules_rust//:workspace.bzl", "bazel_version")
bazel_version(name = "bazel_version")

load("//third_party/cargo:crates.bzl", "raze_fetch_remote_crates")

rust_repositories()
rules_nodejs_dependencies()
raze_fetch_remote_crates()
