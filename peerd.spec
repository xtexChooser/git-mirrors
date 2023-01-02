Name:           peerd
Version:        0.1.1
Release:        0
Summary:        Manage BGP peers with etcd
License:        Apache-2.0
Group:          Productivity/Networking/Other
Url:            https://source.moe/XTEX-VNET/peerd
Source0:        https://source.moe/XTEX-VNET/peerd/archive/%{version}.tar.gz
Source1:        peerd.example.toml
BuildRequires:  protobuf-devel
BuildRequires:  curl
BuildRequires:  gcc

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
cargo build --profile=release

%install
source "$HOME/.cargo/env"
cargo install --path . --root=%{buildroot}%{_prefix}
mkdir -p %{buildroot}/etc/
install -m 655 %{SOURCE1} %{buildroot}/etc/peerd.toml

%check
source "$HOME/.cargo/env"
cargo test

%files
%{_bindir}/peerd
/etc/peerd.toml

%changelog
