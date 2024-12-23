# 🪠 Plunger

[![experimental](http://badges.github.io/stability-badges/dist/experimental.svg)](http://github.com/badges/stability-badges)

Plunger is a small rust tool to parse the workdir of a
[nextflow](https://nextflow.io/) pipeline.
It can measure and aggregate the used disk size per task
and delete a specific one if requested. 

## Install

To install plunger, you will need to [install rust and cargo](https://www.rust-lang.org/tools/install) first. Then run:

```bash
cargo install --git https://github.com/davidmasp/plunger.git
```

Or check the latest
[release page](https://github.com/davidmasp/plunger/releases)
for pre-built (linux) binaries.

## Quickstart

The following use case can be found when
debugging or developing a nextflow pipeline.
In this hypothetical case, the final step
keeps failing. We update the script
and before resuming, we clean the
workdir for that specific task, while keeping
the previous ones intact. This allows us to resume
the execution of the bugging process without
maintaing the files in the workdir.

```bash
$ cd example_pipeline

$ nextflow run main.nf

 N E X T F L O W   ~  version 24.10.1

Launching `main.nf` [disturbed_lichterman] DSL2 - revision: 5d2a31cd6c

executor >  local (8)
[17/db3ef8] A (1) [100%] 4 of 4 ✔
[86/a3da3e] B (1) [100%] 4 of 4 ✔

$ du -sh work 
4.1G    work

$ plunger
B ➡️ 12.7kB
A ➡️ 4.29GB

$ plunger -t A
A ➡️ 4.29GB
B ➡️ 12.7kB

$ du -sh work 
96K     work
```

If developing with "decoy files", this shouldn't be a
major problem, however, sometimes pipelines only
fail for some samples in a large list during "production"
or even a single
sample requires large files per process.

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
time, you should be able to just run plunger alone. See [quickstart](#quickstart)

## Next steps

I need to parse the log files to be able to prune specific runs.
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
are not present in the `deadly_newton` run, aka the last one.
