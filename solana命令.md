### 获取 solana 地址

```
solana address
```

### 获取 solana 配置信息

```
solana config get
```

### 修改 solana 配置

- 修改 solana 连接的网络

  - 使用具体网址
  
    ```
    solana config set --url https://api.devnet.solana.com
    ```
  
  - 使用网络名称
  
    ```
    solana config set --url mainnet-beta
    solana config set --url devnet
    solana config set --url localhost
    solana config set --url testnet
    ```
  
  - 使用缩写参数
  
    ```
    solana config set -um    # For mainnet-beta
    solana config set -ud    # For devnet
    solana config set -ul    # For localhost
    solana config set -ut    # For testnet
    ```

### 查看 solana 余额

```
solana balance
```

