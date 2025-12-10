# rs-broker
mq broker,eg:kafka or redis

# rdkafka
- https://crates.io/crates/rdkafka
- https://github.com/fede1024/rust-rdkafka

# librdkafka
- https://github.com/confluentinc/librdkafka

# usage
please see `examples` or https://github.com/daheige/rs-broker-demo

# install librdkafka
- macos安装方式：
```shell
brew install pkgconf
brew install zlib
brew install librdkafka
```

- apt安装方式：
1. 安装相关依赖
```shell
apt-get install -y build-essential libcurl4-openssl-dev libssl-dev zlib1g-dev pkg-config wget curl
```

2. 源码cmake编译安装
```shell
cd /opt && wget https://github.com/confluentinc/librdkafka/archive/refs/tags/v2.12.1.tar.gz
tar -zxf v2.12.1.tar.gz && cd /opt/librdkafka-2.12.1 && mkdir build && cd build && cmake ..
make && make install
```

3. 设置环境变量
```shell
export PKG_CONFIG_PATH=/usr/local/lib/pkgconfig
export PKG_CONFIG_ALLOW_SYSTEM_LIBS=1
export PKG_CONFIG_ALLOW_SYSTEM_CFLAGS=1
```
建议将上面的环境变量设置放入`~/.bash_profile`文件，然后执行`source ~/.bash_profile`生效。

4. 查看是否安装成功
```shell
pkg-config --modversion rdkafka
```
