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
	($key: ident, list str) => {
		$crate::config!($key, list str, ',');
	};
	($key: ident, list str, $split: expr) => {
		::paste::paste! {
			pub static [<CONFIG_ $key>]: std::sync::LazyLock<Vec<&'static str>> = std::sync::LazyLock::new(|| {
				std::env::var(concat!("SPOCK_", stringify!($key)))
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
	($key: ident, list parse $typ: ty) => {
		$crate::config!($key, list parse $typ: ty, ',');
	};
	($key: ident, list parse $typ: ty, $split: expr) => {
		::paste::paste! {
			pub static [<CONFIG_ $key>]: std::sync::LazyLock<Vec<$typ>> = std::sync::LazyLock::new(|| {
				std::env::var(concat!("SPOCK_", stringify!($key)))
					.unwrap_or_default()
					.split($split)
					.map(|s| s.parse::<$typ>().expect(concat!("could not parse SPOCK_", stringify!($key))))
					.collect()
			});
		}
	};
	($key: ident, list parse $typ: ty, default $default: expr) => {
		$crate::config!($key, list parse $typ: ty, ',', default $default);
	};
	($key: ident, list parse $typ: ty, $split: expr, default $default: expr) => {
		::paste::paste! {
			pub static [<CONFIG_ $key>]: std::sync::LazyLock<Vec<$typ>> = std::sync::LazyLock::new(|| {
				std::env::var(concat!("SPOCK_", stringify!($key)))
					.unwrap_or_else(|| $default.to_string())
					.split($split)
					.map(|s| s.parse::<$typ>().expect(concat!("could not parse SPOCK_", stringify!($key))))
					.collect()
			});
		}
	};
}
