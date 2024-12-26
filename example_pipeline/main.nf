#!/usr/bin/env nextflow

params.seed = 42

process A {
  input: 
    val x
  output:
    path "${x}_file.txt", emit: a
  script:
    """
    # ${params.seed}
    head -c 1G </dev/urandom > ${x}_file.txt
    """
}

process B {
  input: 
    path x
  output:
    path "${x.baseName}_file2.txt.gz"
  script:
    """
    # ${params.seed}
    zcat < ${x} | gzip -c > ${x.baseName}_file2.txt.gz
    """
}

process C {
  input: 
    path x
  output:
    path "${x.baseName}_file3.txt"
  script:
    """
    # ${params.seed}
    zcat < ${x} > ${x.baseName}_file3.txt
    """
}

workflow {
  Channel.of('Bonjour', 'Ciao', 'Hello', 'Hola') | A | B | C
}


