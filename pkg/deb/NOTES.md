# NOTES

The following instructions are for Ubuntu 24.04

## Setup

```shell
rustup upgrade
sudo apt-get -y update
sudo apt-get -y install -y cmake g++ libprotobuf-dev protobuf-compiler
cargo install cargo-deb
```

## Building

```shell
cargo deb
```

## Testing

```shell
sudo dpkg -i target/debian/suzhaobao_1.0.0~beta.9_amd64.deb
```

## Running

#### Start the service
```shell
$ sudo service suzhaobao start
```

#### Stop the service
```shell
$ sudo service suzhaobao stop
```

#### Enable the service
```shell
$ sudo service suzhaobao enable
```

#### Stop the service
```shell
$ sudo service suzhaobao disable
```

#### Get the service status
```shell
$ sudo service suzhaobao status
```

Below is an example response

```shell
● suzhaobao.service - SuZhaoBao Service
     Loaded: loaded (/lib/systemd/system/suzhaobao.service; enabled; vendor preset: enabled)
     Active: active (running) since Thu 2025-06-02 23:34:35 UTC; 5min ago
   Main PID: 23177 (suzhaobao)
      Tasks: 5 (limit: 4605)
     Memory: 3.2M
     CGroup: /system.slice/suzhaobao.service
             └─23177 /usr/share/suzhaobao/suzhaobao start --log info --user root --pass root
```

#### View service logs

```shell
sudo journalctl -f -u suzhaobao
```
