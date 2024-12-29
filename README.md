# 🪠 Plunger

➡️ [Changelog](./CHANGELOG.md)

[![experimental](http://badges.github.io/stability-badges/dist/experimental.svg)](http://github.com/badges/stability-badges)

Plunger is a small collection of tools to parse the workdir of a
[nextflow](https://nextflow.io/) pipeline and perform some cleaning
operations.

## Install

To install plunger, you will need to [install rust and cargo](https://www.rust-lang.org/tools/install) first. Then run:

```bash
cargo install --git https://github.com/davidmasp/plunger.git
```

Or check the latest
[release page](https://github.com/davidmasp/plunger/releases)
for pre-built (linux) binaries.

## Quickstart

After installing, navigate to a nextflow run dir and run:

```bash
$ cd example_pipelinene
$ nextflow run main.nf --seed 42
[blah blah blah]
$ plunger task
B ➡️ 38.2kB
C ➡️ 37.7kB
A ➡️ 12.88GB
```

## Other use cases

### Deleting files from a specific process

This case can remove all folders from a single process in the workdir.
This can be useful when developing pipelines when
a specific process is modified many times and has heavy outputs.

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

$ plunger task
B ➡️ 12.7kB
A ➡️ 4.29GB

$ plunger task -t A
A ➡️ 4.29GB
B ➡️ 12.7kB

$ du -sh work 
96K     work
```

### Measuring disk usage per process

To just measure how much disk space each process is taking, run the same command
without the --tasks argument. We can also use custom workdirs.

Although `--limit-lines` is also an argument, the default value should be enough. It might
be needed in large SLURM configs or untested environments. 

```bash
plunger task --file .command.run --limit-lines 100 ./work
```

### Deleting all but one run

This usecase is equivalent to `nextflow clean -but ...`. Essentially, deletes all
folders that are not relevant for the "latest" run.

Note that although plunger seems faster in this single run test it might lack
internal functionality nextflow clean has (e.g. modifying the history file).
I might try to implement a complete drop-in replacement in the future.

```bash
$ nextflow run main.nf --seed 44 -resume
$ nextflow run main.nf --seed 47 -resume
$ nextflow run main.nf --seed 50 -resume
Launching `main.nf` [modest_mestorf] ... blah ...
$ du -sh .                              
13G     .
$ time nextflow clean -but modest_mestorf -f 
... blah ...
nextflow clean -but modest_mestorf -f  0.86s user 0.15s system 94% cpu 1.073 total
# same as above
$ plunger -v clean
12:14:12 [INFO] Running clean with rundir: .nextflow.log
12:14:12 [INFO] Would purge size: 8.59GB
$ time plunger -v clean -f              
12:15:36 [INFO] Running clean with rundir: .nextflow.log
12:15:36 [INFO] Purged size: 8.59GB
plunger -v clean -f  0.01s user 0.05s system 32% cpu 0.169 total
$ du -sh .
4.1G    .
```
