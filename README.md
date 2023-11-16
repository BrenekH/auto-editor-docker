# Auto Editor Docker

Docker image for [WyattBlue/auto-editor](https://github.com/WyattBlue/auto-editor)

## Features

* Dockerized `auto-editor` CLI (no need to install dependencies)

* Watch folder for continuous bulk processing jobs

## Installation

```bash
docker pull brenekh/auto-editor
```

## Usage

### CLI

```bash
docker run -it -v ./:/videos brenekh/auto-editor /videos/my-video.mp4
```

> Note: In order for `auto-editor` to access the files on your disk, you must pass them as a [volume](https://docs.docker.com/storage/volumes/).
> The above command uses a relative path (`./`) as an alias for the current directory, which is only available in Docker >=23.
> If you are using an older version, you must specify the full path to the folder in which your content resides.

### Watch Mode

```bash
docker run -it -v ./watch:/watch -v ./output:/output brenekh/auto-editor
```

When no arguments are passed to the container, it will instead behave as a daemon watching for new files in the `AUTO_EDITOR_WATCH_DIR` folder.
After the files are processed, they will be moved into the `AUTO_EDITOR_OUTPUT_DIR`.

`AUTO_EDITOR_WATCH_DIR` and `AUTO_EDITOR_OUTPUT_DIR` are environment variables that default to `/watch` and `/output` respectively.
