# benchmark for several helloworld level http server

## machine

```
OS: Arch Linux on Windows 10 x86_64
Kernel: 5.15.90.4-microsoft-standard-WSL2
Uptime: 1 hour, 22 mins
Packages: 435 (pacman)
Shell: bash 5.2.21
Terminal: Windows Terminal
CPU: Intel i5-10200H (8) @ 2.400GHz
GPU: aa44:00:00.0 Microsoft Corporation Basic Render Driver
Memory: 1070MiB / 7877MiB
```

The server and client are on same machine.

## client

```bash
wrk -c 100 -d 20 -t 4 http://127.0.0.1:8081/hello
```

## server

### ae(epoll) tcp

```
Requests/sec: 425484.66
Transfer/sec:     20.29MB
```

### rust_mio tcp

```
Requests/sec: 487706.16
Transfer/sec:     23.26MB
```

### go_fasthttp

```
Requests/sec: 430998.88
Transfer/sec:     60.01MB
```

### rust_async_std tcp

```
Requests/sec: 404329.40
Transfer/sec:     19.28MB
```

### go_std_http

```
Requests/sec: 218872.47
Transfer/sec:     26.72MB
```

### rust_tokio

```
Requests/sec: 172601.84
Transfer/sec:      8.23MB
```
