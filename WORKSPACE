# maven_jar(
#     name = "org_clojure",
#     artifact = "org.clojure:clojure:1.9.0",
#     sha1 = "09ee19f88152373323b8af7be35de5aa6c0c4b2b",
#     sha1_src = "81d88439cbe00268ca07f1b91dcf8ce660856afe",
# )
# maven_jar(
#     name = "org_clojure_spec_alpha",
#     artifact = "org.clojure:spec.alpha:0.2.176",
#     sha1 = "cd2d5b03d2ff95a958cb075201b89d28a7dea626",
#     sha1_src = "cc54a48e7558777f91be730020dfd3676056d301",
# )
# maven_jar(
#     name = "org_clojure_core_specs_alpha",
#     artifact = "org.clojure:core.specs.alpha:0.2.44",
#     sha1 = "6027ceb1d1ae70a6a3fb1a8da2144632fa688604",
#     sha1_src = "0993fa305c2281cd2b77dcbc410bdcc5c67c8686",
# )

#load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
# http_archive(
#     name = "io_bazel_rules_rust",
#     sha256 = "52b5e25d6b0378f9043ea02afbf10b77c1bfce880b1f9f10039618640c7ab57f",
#
#     urls = [
#         "https://github.com/bazelbuild/rules_rust/archive/0.0.7.tar.gz",
#     ],
#     strip_prefix = "rules_rust-0.0.7"
# )

load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository")
git_repository(
  name = "io_bazel_rules_rust",
  commit = "23ab9d5ed7e78ed3c9fd2cf7627eaa6ea60742bf",
  remote = "https://github.com/bazelbuild/rules_rust.git",
)

git_repository(
    name = "bazel_skylib",
    remote = "https://github.com/bazelbuild/bazel-skylib.git",
    tag = "0.6.0",  # change this to use a different release
)

load("@io_bazel_rules_rust//rust:repositories.bzl", "rust_repositories")
rust_repositories()

load("@io_bazel_rules_rust//:workspace.bzl", "bazel_version")
bazel_version(name = "bazel_version")
