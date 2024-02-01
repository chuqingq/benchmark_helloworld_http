# benchmark for several helloworld level http server

## machine

OS: Arch Linux on Windows 10 x86_64
Kernel: 5.15.90.4-microsoft-standard-WSL2
Uptime: 1 hour, 22 mins
Packages: 435 (pacman)
Shell: bash 5.2.21
Terminal: Windows Terminal
CPU: Intel i5-10200H (8) @ 2.400GHz
GPU: aa44:00:00.0 Microsoft Corporation Basic Render Driver
Memory: 1070MiB / 7877MiB


The server and client are on same machine.

## client

```bash
wrk -c 100 -d 20 -t 4 http://127.0.0.1:8081/hello
```

## server

### ae(epoll) tcp

Requests/sec: 435272.56
Transfer/sec:     20.76MB

### go_fasthttp

Requests/sec: 414199.65
Transfer/sec:     57.67MB

### rust_async_std tcp

Requests/sec: 251513.22
Transfer/sec:     11.99MB

### go_std_http

Requests/sec: 216081.88
Transfer/sec:     26.38MB

### rust_tokio

Requests/sec: 169914.21
Transfer/sec:     11.99MB

### rust_mio tcp

Requests/sec:  93668.27
Transfer/sec:      4.47MB

