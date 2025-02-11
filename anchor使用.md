[Anchor开发指南](https://leapwhale.com/article/9dk17j5q)

## 常用指令

### 1 初始化新项目

使用 `anchor init` 命令可以快速创建一个全新的 Anchor 项目模板

```
anchor init my_project
```

- 如果失败报错需要检查 solana 环境以及依赖项是否都已安装或更新

### 2 创建新程序（合约）

使用 `anchor new` 命令在现有项目中创建一个新的智能合约程序

```
anchor new my_program
```



### 3 编译程序

使用 `anchor build` 命令将 Rust 源码转换为 Solana 可执行的二进制文件

```
anchor build [my_project]
```

- 根目录下可省略项目名直接 `anchor build`

