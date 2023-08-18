llvm-rt-src:=$(src)/external/llvm/src/
llvm-rt-out:=$(curout)

llvm-rt-out-user:=$(curout)user/

$(llvm-rt-out-user)Makefile: $(llvm-rt-src)runtimes/ $(llvm-rt-src)libcxx/ $(llvm-rt-src)runtimes/CMakeLists.txt
	$(call action,"GEN  ")
	CC="$(CC)" \
	CXX="$(CC)" \
	CFLAGS="--target=$(CLANG_TARGET) $(mk-cflags) -nostdinc -nostdlib" \
	CXXFLAGS="--target=$(CLANG_TARGET) $(mk-cppflags) -nostdinc -nostdlib -nostdinc++ -nostdlib++" \
	LDFLAGS="-Wl,--build-id=sha1" \
	cmake \
		-S $(llvm-rt-src)runtimes \
		-B $(llvm-rt-out-user) \
		-G "Unix Makefiles" \
		-Wno-dev \
        -DLLVM_ENABLE_RUNTIMES=libcxx \
		-DLIBCXX_INCLUDE_TESTS=fasle \
		-DLLVM_INCLUDE_DOCS=OFF \
		-DLLVM_INCLUDE_TESTS=OFF \
        -DLIBCXX_CXX_ABI=none \
		-DLIBCXX_INCLUDE_BENCHMARKS=OFF \
		-DLIBCXX_INCLUDE_TESTS=OFF \
		-DLIBCXX_ENABLE_STATIC_ABI_LIBRARY=ON \
		-DLIBCXX_ENABLE_SHARED=OFF \
		-DLIBCXX_ENABLE_STATIC=ON \
		-DLIBCXX_ENABLE_RTTI=OFF \
		-DLIBCXX_ENABLE_RANDOM_DEVICE=OFF \
		-DLIBCXX_ENABLE_FILESYSTEM=OFF \
		-DLIBCXX_ENABLE_LOCALIZATION=OFF \
		-DLIBCXX_ENABLE_THREADS=OFF \
		-DLIBCXX_ENABLE_MONOTONIC_CLOCK=OFF \
		
$(llvm-rt-out)%: NO_CFLAGS_ERROR=y

$(llvm-rt-out-user)lib/libc++.a: $(llvm-rt-out-user)Makefile
	$(call action,"MAKE ")
	rm -f $@
	$(MAKE) -C $(llvm-rt-out-user)

libcxx-user-lib:=$(llvm-rt-out-user)lib/libc++.a

