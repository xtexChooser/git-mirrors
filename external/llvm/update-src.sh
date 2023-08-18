#!/usr/bin/bash

rm -rf llvm-project-main/ main.tar.gz
curl -L -O https://github.com/llvm/llvm-project/archive/refs/heads/main.tar.gz
tar -xf main.tar.gz llvm-project-main/{LICENSE.TXT,compiler-rt/{README.txt,LICENSE.TXT,lib/builtins/},libcxx/}
rm -rf llvm-project-main/libcxx/{appveyor*,test/,benchmarks/}
rm -rf src
mv llvm-project-main src
rm main.tar.gz
