# HiSilicon Device Tree Table (HSDT) file unpacker

This tool unpacks HSDT files, which are used by HiSilicon-based devices, for example, smart phones by Huawei.

On Huawei's smart phone devices using HiSilicon SoCs, device-tree table is stored in a partition named `dts`.

The format of HSDT is a bit (strictly speaking, 96 bits) simliar to QCDT files, which are used by Qualcomm devices. HSDT files begin with a header (`dt_table_t`) following by many entries (`dt_entry_t`). A magic value of 0x54445348 (`HSDT` in ASCII octects) is always placed at the head of HSDT files.

This tool is able to classify HSDT extracted from Huawei system update package (`UPDATE.APP`), which has an extra header of 4 KiB at the beginning, and HSDT dumped from Huawei smart phones (`/dev/block/bootdevice/by-name/dts`). It can also GZIP-decompress extracted DTB files automatically and extract VRL data from the table (the VRL data is related to verification, but the VRL-extracting feature is not test, as I did not find any HSDT file with VRL data in my reverse engineering project of their fastboot).

To build this tools, run `make` with Zig tool-chain configured.

There is also a pattern of ImHex editor available at [here](https://github.com/WerWolv/ImHex-Patterns/pull/280).

To unpack Huawei's update pack, the `UPDATE.APP` file, checkout [this](https://github.com/echo-devim/huextract) or [this](https://github.com/Project-Satori/HuaweiUpdateExtractor) or [this](https://github.com/xjljian/huawei_UPDATE.APP_unpacktool).

> This is free and unencumbered software released into the public domain.
>
> Anyone is free to copy, modify, publish, use, compile, sell, or
> distribute this software, either in source code form or as a compiled
> binary, for any purpose, commercial or non-commercial, and by any
> means.
>
> In jurisdictions that recognize copyright laws, the author or authors
> of this software dedicate any and all copyright interest in the
> software to the public domain. We make this dedication for the benefit
> of the public at large and to the detriment of our heirs and
> successors. We intend this dedication to be an overt act of
> relinquishment in perpetuity of all present and future rights to this
> software under copyright law.
>
> THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
> EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
> MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
> IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
> OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
> ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
> OTHER DEALINGS IN THE SOFTWARE.
>
> For more information, please refer to <https://unlicense.org>