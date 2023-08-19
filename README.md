# xtex-os

> A experimental operating system.

![CI Status](https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fci.codeberg.org%2Fapi%2Frepos%2F12521%2Fpipelines%3Fpage%3D1%26perPage%3D1&query=%24.0.status&style=flat-square&label=build) [![license](https://img.shields.io/badge/license-Apache--2.0-blue?style=flat-square)](http://www.apache.org/licenses/LICENSE-2.0)

## Download

The development is on the [git repository on Codeberg](https://codeberg.org/xtex/xtex-os).

To get the source, clone `git@codeberg.org:xtex/xtex-os.git` with git.

### Prebuilt

The prebuilt binaries are built by [Codeberg CI](https://ci.codeberg.org/) and uploaded to [envs.sh](https://envs.sh).

To get the prebuilt binaries, you can access the [download API](https://xtex.p.projectsegfau.lt/xos-download.sh?source).

For example, the x86 ISO is at [here](https://xtex.p.projectsegfau.lt/xos-download.sh?x86-iso).

## Build

1. Create a file called `local.config.mk` in the source code. Then write the following content to it.

   ```makefile
   CONFIG_ARCH = x86
   ```

   To debug, you can also add `CONFIG_NO_KASLR = y` and `CONFIG_DEBUG = y`.

2. Run `make -j4`
3. Run `make qemu-iso` to start qemu. Or `make debug`.

## License

This project is licensed under the Apache License, Version 2.0.

A copy of the license is at the LICENSE file.

>    Copyright 2023 xtex
>
>    Licensed under the Apache License, Version 2.0 (the "License");
>    you may not use this file except in compliance with the License.
>    You may obtain a copy of the License at
>
> ​       http://www.apache.org/licenses/LICENSE-2.0
>
>    Unless required by applicable law or agreed to in writing, software
>    distributed under the License is distributed on an "AS IS" BASIS,
>    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
>    See the License for the specific language governing permissions and
>    limitations under the License.

