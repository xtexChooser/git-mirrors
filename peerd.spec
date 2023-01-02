Name:           peerd
Version:        0.1.1
Release:        0
Summary:        Manage BGP peers with etcd
License:        Apache-2.0
Group:          Productivity/Networking/Other
Url:            https://source.moe/XTEX-VNET/peerd
Source0:        https://source.moe/XTEX-VNET/peerd/archive/%{version}.tar.gz
BuildRequires:  protobuf-compiler
BuildRequires:  curl

%description
Manage BGP peers with etcd

%prep
%setup -q -n peerd
rm -rf .cargo
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain nightly -y
source "$HOME/.cargo/env"
cargo --version

%build
source "$HOME/.cargo/env"
cargo build

%install
source "$HOME/.cargo/env"
cargo install
# install -D -d -m 0755 %{buildroot}%{_bindir}
# install -m 0755 %{_builddir}/%{name}-%{version}/target/release/hellorust %{buildroot}%{_bindir}/hellorust
 
%check

%files
%{_bindir}/peerd

%changelog
