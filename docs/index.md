# Tiny Tiny Web 2 临时文档
这是一个用以简单创建Web服务器的软件，使用 Rust 语言开发， GPLv3 开源，以下简称 TTWeb 。

本文档所述内容可能不适合最新版本或对于最新版本而言不全面。
当前文档基于版本 2.0.0-alpha2。

### 开始使用：
首先，在程序根目录下创建文件夹：config、export、temp。

用处：
1. config：存放程序的配置文件
2. export：存放网页的源代码
3. temp：存放临时文件

然后，在config目录下创建 main.gc 文件，配置文件的读取从这里开始。

在 main.gc 文件中写入如下内容：
```
$ +addr 127.0.0.1:22397
+ index.html /
```
然后，在 export 文件下创建 index.html 文件
在里面写入如下内容：
```
Hello, World!
```
然后启动程序，在浏览器内打开 http://127.0.0.1:22397/
如果一切顺利，你应该会看到打印：
```
Hello, World!
```

### 所有指令

```
# 挂载一个文件到一个URL,后两个选项是可选的，如果要挂载到根路径，应该使用“/”
+ index.html index.html text/html;charset=utf-8 

# 删除一个URL，这个示例删除了对 index.html 路径的绑定，但是并没有删除 index.html 文件
- index.html

# 对一个内部变量进行设置，在这个例子中是设置使用本地时间而非世界标准时间，此外，这也是默认设置
$ localtime yes
# 以下是除此之外所有的默认设置：
$ debug no
$ threads 2
$ xrps-counter-cache-size 8
$ box-num-per-thread-mag 1.0
$ box-num-per-thread-init-mag 1.0
$ xrps-predict-mag 1.1
# 在所有涉及小数的配置中，最多使用三位小数

# 设置一个 404 页面，当请求的网页不存在时返回给浏览器这个页面
$ 404page 404.html

# 添加一个监听地址
$ +addr 127.0.0.1:80
$ +addr [fe80::1]:80

# 导入并加载一个配置文件
@ a.gc

# 编译一个文件，与下面的加载命令要一起使用，对于要替换的位置，使用 $_gcflag 占位符
compile contents.html
# 注入一个文件（用 a.txt, b.txt, c.txt 中的内容替换 contents.html 中的 $_gcflag 占位符）
inject contents.html a.txt b.txt c.txt
```