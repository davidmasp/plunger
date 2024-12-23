# 🪠 Plunger

Plunger is a small rust tool to parse the workdir of a [nextflow](https://nextflow.io/) pipeline.
It can measure and aggregate the used disk size per task
and delete a specific one if requested. 

## Install

To install plunger, you will need to [install rust and cargo](https://www.rust-lang.org/tools/install) first.

```bash
cargo install --git https://github.com/davidmasp/plunger.git
```

## Quickstart

```bash
$ cd example_pipeline

$ nextflow run main.nf

$ plunger
```

## Other use cases

### Measuring disk usage in custom workdirs

To just count how much disk space each task is taking, run the same command
without the --tasks argument.

```bash
plunger --file .command.run --limit-lines 100 ./work
```

### Purging a specific task from the workdir

```bash
plunger --tasks SUBSAMPLE:SEQTK_SUBSAMPLE --file .command.run --limit-lines 100 ./work
```

File, limit-lines and the workdir path have all sensible defaults so, most of the
time, you should be able to just run plunger alone.

## Next steps

I need to parse the log files, to be able to prune specific runs.
The use case here would be. For a pipeline A -> B -> C where
you have been debugging the pipeline many times without cleaning.
Thus, if the bug was on B, many B directories contain intermediate
files that you will never rescue again because they are not
resumable. However, you don't want to start a fresh run.

If you simply delete task B, then you will loose the
latest run which contains the correct, up-to-date data.

In such escenario, it would be cool to run:

```bash
plunger --keep-run deadly_newton -f
```

This would remove all the work dir subdirectories that
are not present in the deadly_newton run.
