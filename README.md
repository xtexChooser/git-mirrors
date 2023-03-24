# Mekbuda Database

> The registration database for [mekbuda](https://codeberg.org/xvnet/mekbuda) run by XV-NET.

## Data

Each file under `data/` is named with a 12-digit lowercase hexadecimal integer, and is mapped to `fd00:443a:ef14:4:XXXX:XXXX:XXXX::`.

For example, `data/000000000001` means `fd00:443a:ef14:4::1:` (or `fd00:443a:ef14:4:0000:0000:0001::`).

Each line of the file represents an address on the routing path and the corresponding PTR record content.

Lines starts with `#` will be ignored.

Empty lines without `\asis(SPACE)` will be ignored.

Lines starts with`\dns(SPACE)` will be parsed as a FQDN (must end with a `.`).

Data is cached for 30 seconds, so changes that have just been committed may not be reflected immediately.

## Updates

All XV-NET members have direct push permissions to this repository.

Do NOT modify any files outside the `data/` directory without the OPs' permission.

All commits must be validly signed (with GPG or SSH).

If the author (i.e. the first committer) does not explicitly state "PUBLIC EDITABLE" in the comments, others should not modify the content of the file without the author's consent.

**For non-XV-NET members**, please create a pull request. Please note that your files must be marked "PUBLIC EDITABLE" and may be removed at any time.

## Licensing

Refer to [LICENSE.md](https://codeberg.org/xvnet/mekbuda-db/src/branch/main/LICENSE.md), CC0, The Unlicense, CC-BY 4.0, CC-BY-SA 4.0 can be choose by the author.

Authors should declare the chosen license in the comments, otherwise the data will be released under CC-BY-SA-4.0.
