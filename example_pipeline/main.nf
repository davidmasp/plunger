#!/usr/bin/env nextflow

process A {
  input: 
    val x
  output:
    path "${x}_file.txt", emit: a
  script:
    """
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
    zcat < ${x} | gzip -c > ${x.baseName}_file2.txt.gz
    """
}

workflow {
  Channel.of('Bonjour', 'Ciao', 'Hello', 'Hola') | A | B
}


