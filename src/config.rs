#[macro_export]
macro_rules! config {
	($key: ident, str, required) => {
		::paste::paste! {
			pub static [<CONFIG_ $key>]: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| {
				std::env::var(concat!("SPOCK_", stringify!($key)))
					.expect(concat!("SPOCK_", stringify!($key), " is missing"))
					.leak()
			});
		}
	};
	($key: ident, str, optional) => {
		::paste::paste! {
			pub static [<CONFIG_ $key>]: std::sync::LazyLock<Option<&'static str>> = std::sync::LazyLock::new(|| {
				std::env::var(concat!("SPOCK_", stringify!($key)))
					.ok()
					.map(|s| &*s.to_string().leak())
			});
		}
	};
	($key: ident, str, $default: literal) => {
		::paste::paste! {
			pub static [<CONFIG_ $key>]: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| {
				std::env::var(concat!("SPOCK_", stringify!($key)))
					.ok()
					.map(|s| &*s.to_string().leak())
					.unwrap_or($default)
			});
		}
	};
	($key: ident, list) => {
		$crate::config!($key, list, ',');
	};
	($key: ident, list, $split: expr) => {
		::paste::paste! {
			pub static [<CONFIG_ $key>]: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
				env::var(concat!("SPOCK_", stringify!($key)))
					.unwrap_or_default()
					.split($split)
					.map(|s| &*s.to_string().leak())
					.collect()
			});
		}
	};
	($key: ident, parse $typ: ty, required) => {
		::paste::paste! {
			pub static [<CONFIG_ $key>]: std::sync::LazyLock<$typ> = std::sync::LazyLock::new(|| {
				std::env::var(concat!("SPOCK_", stringify!($key)))
					.expect(concat!("SPOCK_", stringify!($key), " is missing"))
					.parse::<$typ>()
					.expect(concat!("could not parse SPOCK_", stringify!($key)))
			});
		}
	};
	($key: ident, parse $typ: ty, optional) => {
		::paste::paste! {
			pub static [<CONFIG_ $key>]: std::sync::LazyLock<Option<$typ>> = std::sync::LazyLock::new(|| {
				std::env::var(concat!("SPOCK_", stringify!($key)))
					.ok()
					.map(|s| s.parse::<$typ>().expect(concat!("could not parse SPOCK_", stringify!($key))))
			});
		}
	};
	($key: ident, parse $typ: ty, $default: expr) => {
		::paste::paste! {
			pub static [<CONFIG_ $key>]: std::sync::LazyLock<$typ> = std::sync::LazyLock::new(|| {
				std::env::var(concat!("SPOCK_", stringify!($key)))
					.ok()
					.map(|s| s.parse::<$typ>().expect(concat!("could not parse SPOCK_", stringify!($key))))
					.unwrap_or($default)
			});
		}
	};
}
