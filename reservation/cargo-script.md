### cargo的一些使用

- cargo add prost -p abi  # 添加prost到abi的cargo下面
- cargo add tonic-build --build -p abi  # --build添加到[build-dependencies]下面
- cargo add tonic -p abi --features gzip  # 添加tonic到abi的cargo下面并且添加features
- cargo add sqlx --features runtime-tokio-rustls --features postgres --features chrono --features json  -p reservation sqlx需要添加这些features