# ActivityStreams Types
__A base set of types from the Activity Streams specification.__

- [Read the documentation on docs.rs](https://docs.rs/activitystreams-types)
- [Find the crate on crates.io](https://crates.io/crates/activitystreams-types)
- [Hit me up on Mastodon](https://asonix.dog/@asonix)

## Usage
First, add the crate to your `Cargo.toml`
```toml
# Cargo.toml

activitystreams-types = "0.4"
```

Then use it in your project!
```rust
// in your project

use activitystreams_types::{context, link::Mention};

fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// A Mention is the only predefined Link type in the Activity Streams spec
    let mut mention = Mention::default();
    mention.as_ref().set_context_xsd_any_uri(context())?;

    let mention_string = serde_json::to_string(&mention)?;

    let mention: Mention = serde_json::from_str(&mention_string)?;

    Ok(())
}
```

## Contributing
Feel free to open issues for anything you find an issue with. Please note that any contributed code will be licensed under the GPLv3.

## License

Copyright © 2020 Riley Trautman

ActivityStreams Types is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

ActivityStreams Types is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details. This file is part of ActivityStreams Types.

You should have received a copy of the GNU General Public License along with ActivityStreams Types. If not, see [http://www.gnu.org/licenses/](http://www.gnu.org/licenses/).
