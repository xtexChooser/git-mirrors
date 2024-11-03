release:
    @maint/publish.sh

build-client:
    @cargo build -p yjyz-tools --target x86_64-pc-windows-gnu

build-client-release:
    @cargo build -p yjyz-tools --target x86_64-pc-windows-gnu --release
