# Building
### Dependencies
We use rust bindings for bluez, a bluethooth library for linux.
- Fedora(-based)
```
  sudo dnf in bluez-libs-devel dbus-devel pkgconf-pkg-config
```

- Debian(-based)
```
  sudo apt install libbluethooth-dev libdbus-1-dev pkg-config
```

### Building

```
   cargo b -r
```
The Binary will be in `target/release/we_golf_server`

you can also just run it with

```
   cargo r -r
```

For development omit the `-r` or `--release` flag for fast builds

sudo python three-panel-golf.py --led-cols=64 --led-chain=3 --led-slowdown-gpio=5
