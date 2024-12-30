# 我的Rust学习历程
## 抛弃DNS解析, 直连才是未来(狗头

人生第一关, 访问到`Github`(笑

```bash
nslookup github.com
---
Server:         127.0.0.53
Address:        127.0.0.53#53

Name:   github.com
Address: 140.82.114.3
```

```bash
nslookup github.global.ssl.fastly.net
---
Server:         127.0.0.53
Address:        127.0.0.53#53

Name:   github.global.ssl.fastly.net
Address: 151.101.193.194
```

```bash
sudo nvim /etc/hosts
```

![DirectConnect](/Picture/DirectConnection.png)

- 找到`github.com`和`github.global.ssl.fastly.net`的ip地址, 修改本地hosts实现绕过DNS直连Github

## 加油🥳🥳🥳

