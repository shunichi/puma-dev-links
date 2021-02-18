# puma-dev link helper

## Install

If you don't have rust compiler, install it.
https://www.rust-lang.org/tools/install


```sh
$ cargo install --git https://github.com/shunichi/puma-dev-links
```

`pdl` command will be installed to `$HOME/.cargo/bin`

## Usage

### Show app list
`pdl list` (or just `pdl`) shows apps in your puma-dev directory (`~/.puma-dev`).

```sh
$ pdl list
my-app1 -> /path/to/my-app1
my-app2 3000
my-app3 3001
```

### Show app port

```sh
$ cd /path/to/my-app3
$ pdl port
3001
```

You can specify app name.

```sh
$ pdl port my-app2
3000
```

If you have Procfile's, you can use `pdl port` in them.

```Procfile
web: ./bin/rails s -p `pdl port`
```

### Link app
`pdl link` links app to the first available port greater than or equal to 3000.

```sh
$ cd /path/to/my-app4
$ pdl link
'my-app4' is linked to port 3002
$ pdl list
my-app1 -> /path/to/my-app1
my-app2 3000
my-app3 3001
my-app4 3002
```

### Unlink app

```sh
$ cd /path/to/my-app3
$ pdl list
my-app1 -> /path/to/my-app1
my-app2 3000
my-app3 3001
my-app4 3002

$ pdl unlink
'my-app3' is unlinked

$ pdl list
my-app1 -> /path/to/my-app1
my-app2 3000
my-app4 3002
```

## Unsupported features
* puma-dev directories other than '~/.puma-dev'
* Subdirectories like `~/.puma-dev/cool/frontend`
* Proxy to other host like `10.3.1.2:9292`
