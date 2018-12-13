# Cargo Cargo-raze and Bazel
This third party directory contains a Cargo.toml file that is used by
Cargo-raze to produce Bazel BUILD files and dependencies we can later use.

It is currently built for remote mode, in a production environment we'd
likely use vendor mode.

When new dependencies are added to Cargo.toml, we must update the remote
resources and build targets.
``` $ cd third_party/cargo
    $ cargo raze
    $ cd ../..
```
