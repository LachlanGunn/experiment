Experiment data management tool
===============================

The `experiment` tool provides a basic utility for the management of
experimental data. It allows experimental data to be stored and then atomically
committed to a data repository.

Synopsis
--------

```console
$ export EXPERIMENT_STORAGE_ROOT=$(mktemp -d)
$ export EXPERIMENT_PATH=$(experiment start some-experiment-type)
$ echo 'Bar' > $(experiment file foo/bar)
$ echo 'Baz' > $(experiment file foo/baz)
$ experiment commit
$ ls $EXPERIMENT_STORAGE_ROOT
some-experiment-type-2018-10-15T08:42:43.882263701+00:00-f0dbf20a
$ ls $EXPERIMENT_STORAGE_ROOT/*
foo  name  start-time
```

Building `experiment`
---------------------

`experiment` is written in Rust, and is build with the `cargo` tool. The easiest
way to install Rust is with the [`rustup`](https://rustup.rs/) tool.  Then, simply run

```console
$ cargo build
$ cargo install
```

License
-------

This code is released under the Apache 2.0 licence.

&copy; 2018 [Secure Systems Group, Aalto University](https://ssg.aalto.fi/).