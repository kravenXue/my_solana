### solana 环境搭建指南

[Install the Solana CLI and Anchor | Solana](https://solana.com/zh/docs/intro/installation)

根据开发文档搭建 solana 开发所需环境

### anchor 安装

avm 允许你在系统上安装和管理不同版本的 Anchor

并可以更轻松地更新 Anchor 版本

- avm 安装命令

  ```
  cargo install --git https://github.com/coral-xyz/anchor avm --force
  ```

  - 使用 wsl 执行可能报错，需要安装或更新 linux 依赖项

    ```
    sudo apt-get update
    ```

    ```
    sudo apt-get install -y \
        build-essential \
        pkg-config \
        libudev-dev llvm libclang-dev \
        protobuf-compiler libssl-dev
    ```

- 检查 avm 安装是否成功

  ```
  avm --version
  ```

- 使用 avm 安装 anchor 框架

  - 安装最新 anchor 框架

    ```
    avm install latest
    avm use latest
    ```

  - 安装指定版本的 anchor 框架

    ```
    avm install 0.30.1
    avm use 0.30.1
    ```

- 检查 anchor 是否安装成功

  ```
  anchor --version
  ```

  