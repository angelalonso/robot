# Creating module for python
- On Cargo.toml
```
[lib]
crate-type = ["cdylib"]

[package.metadata.maturin]
classifier = [
  "Programming Language :: Rust",
  "Operating System :: POSIX :: Linux",
]
```
$ python3 -m venv env
$ source env/bin/activate
(env) $ pip3 install maturin
(env) $ maturin develop
