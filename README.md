# exif-rename

A CLI to rename image/video files by Exif data from exiftool

## Usage

```
Usage: exif-rename --exif <EXIF> --pattern <PATTERN> [FILE]

Arguments:
  [FILE]  File to rename

Options:
  -e, --exif <EXIF>        Exif filename
  -p, --pattern <PATTERN>  filename pattern
  -h, --help               Print help
  -V, --version            Print version
```

```shell
$ exiftool -s IMG_1234.JPG > exif.txt
$ exif-rename --exif exif.txt --pattern "{y}{m}{D}_{t}_{T2}_{r}.{e}" IMG_1234.JPG
```

### Patterns

Uses Downloader Pro Pattern. See [Downloader Pro Manual](http://www.breezesys.com/downloads/Downloader_Pro_Manual.pdf).

| token | description                                   |
|-------|-----------------------------------------------|
| {Y}   | 4-digit year                                  |
| {y}   | 2-digit year                                  |
| {m}   | month (01-12)                                 |
| {D}   | day of the month (01-31)                      |
| {t}   | time HHMMSS                                   |
| {H}   | hour (00-23)                                  |
| {h}   | hour (01-12)                                  |
| {M}   | minutes (00-59)                               |
| {S}   | seconds (00-59)                               |
| {W}   | week number                                   |
| {a}   | abbreviated weekday name (e.g. Fri)           |
| {f}   | image name (e.g. 'IMG_' for 'IMG_1234.JPG')   |
| {r}   | image number (e.g. '1234' for 'IMG_1234.JPG') |
| {e}   | extension (e.g. 'JPG' for 'IMG_1234.JPG')     |
| {T2}  | fill camera model name (e.g. 'Canon EOS 5D')  |

## Build

```shell
$ cargo build --release
```

You can find executable in `target/release/` directory.
