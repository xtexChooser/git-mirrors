# Tianguan - a lightweight, parallel ssh tool

- ⚡️ Lightweight and fast. Only one file

- 🔋 Powerful. Extensible and welcome for contributions.

---

## Use

### Installation

- Run without installation: `bash <(curl -sSL https://codeberg.org/xens/tianguan/raw/branch/main/tiang) --help` (for bash, zsh, etc.), `bash (curl -sSL https://codeberg.org/xens/tianguan/raw/branch/main/tiang | psub) --help` (for fish)

- Install locally:

  ```bash
  curl -SL https://codeberg.org/xens/tianguan/raw/branch/main/tiang -o /usr/local/bin/tiang
  chmod +x tiang
  ./tiang --help
  ```

- Add to git:

  ```bash
  git submodule add --name tianguan https://codeberg.org/xens/tianguan.git tianguan
  ln -s tianguan/tiang tiang
  ./tiang --help
  ```

  or subtree:

  ```bash
  git subtree add --prefix tianguan https://codeberg.org/xens/tianguan.git main
  ln -s tianguan/tiang tiang
  ./tiang --help
  ```

### Write profile scripts

Tianguan searches and sources profile scripts at `{.tianguan,.tianguan/profile}.{sh,bash}` and `.tianguan.profile.sh`.

An examples is available [here](https://codeberg.org/xens/tianguan/src/branch/main/.tianguan.example.sh).

The minimal profile script only contains required `tiang::target` commands which declares target servers.

### Run commands

Tianguan handles all command line arguments one by one.

For example: `./tiang -f ".ext | not" -k -c "neofetch"` will execute neofetch on all targets with `ext=false` metadata.

`tiang` first loads all targets, then `-f` applies a [jq filter](https://jqlang.github.io/jq/manual/) which removes targets that is `ext=true`.

Then, `-k` asks user for confirmation for the target list and `-c` send a command to targets.

---

## License

> Copyright 2023 Xensor V Network
>
> Licensed under the Apache License, Version 2.0 (the "License");
>
> you may not use this file except in compliance with the License.
>
> You may obtain a copy of the License at
>
> <http://www.apache.org/licenses/LICENSE-2.0>
>
> Unless required by applicable law or agreed to in writing, software
>
> distributed under the License is distributed on an "AS IS" BASIS,
>
> WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
>
> See the License for the specific language governing permissions and
>
> limitations under the License.
