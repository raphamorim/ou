# ou

`ou` helps you serve a static site, single page application or just a static file (no matter if on your device or on the local network). It also provides a neat interface for listing the directory's contents.

Pre-built binaries are available on the [Github Releases tab](https://github.com/raphamorim/ou/releases).

You can use cargo to install:

```
$ cargo install ou
```

With cargo-binstall:

```sh
$ cargo binstall ou
```

## Quick example

![Tree](resources/tree.png)

Once `ou` is installed, you can run this command inside your project's directory. It will create by default in `8000` port:

```
$ ou
```

You can also specify the port by using --port

```
$ ou --port 3000
```

You can also specify the path

```
$ ou ../path-to-site --port 8123
```

Result:

![Result ou](resources/demo.png)
