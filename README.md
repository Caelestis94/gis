# GIS - Git Identity Switcher

I made this CLI tool to learn more about Rust and make it easier for me to switch between git identities (Author and Email) on my machine. This is my first real project in Rust.

## Installation

This is used on a Mac, probably won't work with other OS's.

```bash
cargo install --git https://github.com/Caelestis94/gis.git
```

## Usage

By default the config is stored in `~/.gisrc`. You can load your own config file with the `--config` flag.

### Add a new identity

```bash
gis identity add "John Doe some@email.com"
```

### List all identities

```bash
gis identity list
```

#### Output

```bash
1. Author : John Doe, Email : some@email.com
2. ...
```

### Switch to an identity

The number is the index of the identity in the list.

```bash
gis switch 1
```

### Remove an identity

The number is the index of the identity in the list.

```bash
gis identity remove 1
```

### Add a workspace

This will add a workspace with the current identity attached to it.

```bash
gis workspace add "workspace name"
```

or for the current directory

```bash
gis workspace add .
```

### List all workspaces

```bash
gis workspace list
```

#### Output

```bash
1. /path/to/workspace > Author : John Doe, Email : some@email.com
2. ...
```

### Remove a workspace

The number is the index of the workspace in the list.

```bash
gis workspace remove 1
```

### Check the current identity

```bash
gis current
```

### Check if the current identity is the same as the workspace

This will check if the current identity is the same as the one attached to the workspace. And if not it will switch to the identity attached to the workspace. (This is how I use it in my .zshrc so I don't have to worry about switching identities when I switch workspaces).

```bash
gis check
```
