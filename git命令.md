### 克隆项目到本地

`git clone`

```
git clone https://github.com/kravenXue/my_solana.git
```

可能会因为 SSL 证书问题导致 clone 失败

可以暂时关闭 ssl 认证，并在 clone 完毕后及时打开

关闭 ssl 认证：

```
git config --global http.sslVerify false
```

打开 ssl 认证：

```
git config --global http.sslVerify true
```

### 添加文件到 commit

`git add`

**example：**

添加所有文件

```
git add .
```

### 提交 commit

```
git commit -m "提交描述"
```

