# How to run unit tests

## Ubuntu

### Dependency
- firefox
- geckodriver
```sh
sudo apt update
sudo apt install firefox firefox-geckodriver -y
```

### Run Unit Tests
```sh
wasm-pack test --headless --firefox
```

## Docker

```
docker build --target test .
```
