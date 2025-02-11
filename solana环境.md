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

- solana 还需要 node.js 和 yarn 环境

- Node.js 安装

  - 通过 nvm 来安装 node.js

  - 先安装 nvm

    ```
    curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/master/install.sh | bash
    ```

  - 可能遇到 SSL 认证报错

    ```
    curl: (60) SSL certificate problem: unable to get local issuer certificate
    ```

  - 添加 -k 参数临时规避 SSL 证书认证

    ```
    curl -k -o- https://raw.githubusercontent.com/nvm-sh/nvm/master/install.sh | bash
    ```

    - 依旧报错

      ```
      fatal: unable to access 'https://github.com/nvm-sh/nvm.git/': server certificate verification failed.
      ```

    - 确保网络环境安全下，临时禁用 git 的 SSL 认证

      ```
      git config --global http.sslVerify false
      ```

    - 后重新执行安装命令

      ```
      curl -k -o- https://raw.githubusercontent.com/nvm-sh/nvm/master/install.sh | bash
      ```

    - 安装后重新开启 git 的 SSL 认证

      ```
      git config --global http.sslVerify true
      ```

  - 重启 terminal

  - 检查 nvm 是否安装成功

    ```
    command -v nvm
    ```

  - nvm 安装 node.js

    ```
    nvm install node
    ```

  - node.js 检查

    ```
    node --version
    ```

- yarn 安装

  - 安装命令

    ```
    npm install --global yarn
    ```

  - yarn 检查

    ```
    yarn --version
    ```

    

  