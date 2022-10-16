# WIK-DPS-TP01

## Pre-request
- Having [Rust](https://www.rust-lang.org/tools/install) & [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed
- Clone the repo !
    - `git clone https://github.com/zHatsuharu/WIK-DPS-TP01.git`

## How to start the project ?
Go in the folder `tp-01` and run the project :
```bash
> cd tp-01
tp-01> cargo run
```
The webserver will launch at `localhost:8080`

You can change the port by editing the environment variable `PING_LISTEN_PORT` before launching it :
```BASH
export PING_LISTENPOST=<YOUR NEW PORT>
```

## How it works ?
Open `localhost:8080/ping` in your browser or curl it with `curl -v localhost:8080/ping`

If you go in another route, you will have a 404 Error like `localhost:8080`.