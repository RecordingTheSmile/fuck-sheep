# 羊了个羊刷通关程序
一个用Rust+Tokio完成的高性能刷通关程序。
# 使用说明
1. 使用抓包工具抓包获取请求token

Android推荐使用小黄鸟，iPhone请使用Stream，Windows请使用Fiddler，Charles等支持HTTPS抓包的工具。

推荐使用iPhone抓包，配置最简单。具体使用方法可以参考网上的教程（后期补上）。

2. 下载Release中的安装包。

3. 启动程序。
启动参数：
```shell
./fuck-sheep -t 填写上面抓包获取到的Token -s 要刷的次数
```

# 拓展
如果您想控制并发数量，请使用-p参数。

如果您不了解这个选项的意义，建议您不要填写。

建议尽可能的减小该数值。