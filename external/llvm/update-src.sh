#!/usr/bin/bash

rm -rf llvm-project-main/ main.tar.gz
curl -L -O https://kgithub.com/llvm/llvm-project/archive/refs/heads/main.tar.gz
tar -xvf main.tar.gz llvm-project-main/{LICENSE.TXT,compiler-rt/{README.txt,LICENSE.TXT,lib/builtins/},libcxx/,runtimes/,cmake/,llvm/cmake/}
rm -rf llvm-project-main/libcxx/{appveyor*,test/,benchmarks/}
rm -rf src
mv llvm-project-main src
rm main.tar.gz
