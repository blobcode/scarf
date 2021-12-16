# 🧣 scarf

scarf is a super simple tcp load balancer based on rust built as a learning project to work with [pine](https://github.com/blobcode/pine), a super simple reverse proxy.

## Getting Started <a name = "getting_started"></a>

To get started, download the project 

```
git clone https://github.com/blobcode/scarf
```

and `cd` into it.

```
cd scarf
```
You can then compile and run.
```
cargo run --release
```

You can also give it a custom path to a config file.
```
cargo run --release ./path/to/scarf.toml
```

## Usage <a name = "usage"></a>
Scarf is meant to be used behind a [reverse proxy](https://github.com/blobcode/pine) / ssl terminator to act as an entrypoint to a service. You can look at an example config file in [scarf.toml](https://github.com/blobcode/scarf/blob/master/scarf.toml).
