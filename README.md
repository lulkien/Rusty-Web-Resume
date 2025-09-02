# Rusty Web Resume

Why rely on a resume creator website that can take your information when you can make your own?

Don't worry, I got you.

## What you need?

- [Trunk](https://github.com/trunk-rs/trunk)
- Rustup toolchain
- Python (optional)

## How to build?

- Install rustup from your favorite package manager.
- Install trunk.
- Install rustup stable toolchain

  ```bash
  rustup default stable
  ```
- Install target wasm32

  ```bash
  rustup target add wasm32-unknown-unknown
  ```
- Modify `config/resume.toml`
- Change your avatar.
- Compile

  ```bash
  trunk build --release
  ```

## How to serve?

There are 2 ways:

- **Method 1**: Run `trunk serve --release` in source code directory.
- **Method 2**: Run `python3 -m http.server 8080` in directory `dist`.

Both ways will serve this resume at port 8080.

## How to let other people view my resume?

### Method 1: Using VPS

You could rent a VPS > Open port 8080 in your firewall > Serve > Other people can access via VPS IP and port.

This method is risky, not recommend.

### Method 2: Using cloudflared

Please follow this [guide](https://www.youtube.com/watch?v=ey4u7OUAF3c)

It's safe and easy, highly recommend.

**Enjoy!**
