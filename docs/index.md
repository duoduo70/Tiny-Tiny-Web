# Tiny Tiny Web 中文文档
这是一个用以简单创建Web服务器的软件，使用 D 语言开发， GPLv3 开源，以下简称 TTWeb 。

本文档所述内容可能不适合最新版本。
当前文档基于版本 2023-2-26 (1.1 过渡到 1.2 之间的开发版本) 。
Grimoire 版本：commit-5668692

配置文件：

```json5
"Listen": ["127.0.0.1:8090","[::]"],    // 监听端口，支持IPv4和IPv6
"ReturnCode": 0,                        // 设置程序返回值，不懂别乱设
"Workers": 4,                           // 设置进程数
"MaxWorkers": 4,                        // 设置最大进程数
"MinWorkers": 4,                        // 设置最小进程数
"MaxWorkerLifetime": 1,                 // 设置进程最大存活时间（小时计），设太高会假死！
"MaxWorkerIdling": 6,                   // 设置最大空转时间（小时计）
"MaxRequestTime": 1,                    // 设置最大请求时间（秒计）
"HttpTimeout": 1,                       // 设置HTTP超时时间（秒计）
"ListenerBacklog": 256,                 // 保存未完成的连接请求数量
"MaxRequestSize": 10485760,             // 设置最大请求大小（字节计）
"KeepAlive": true,                      // 设置链接是否会保持存活
"KeepAliveTimeout": 3,                  // 设置最大存活时间（秒计）

"WorkerUser": "",                       // 设置进程用户（仅Posix）
"WorkerGroup": "",                      // 设置进程用户组（仅Posix）

"ExtraFileStorages": [""],              // 额外文件存储库(例如"D:/abc")（开放设定的文件夹）
"EnableDistributivePage": false,        // 让额外文件存储库中也可以使用默认路由来路由网页
"StaticStorages": [""],                 // 静态存储库，在默认路由中，会将文件夹渲染成类似FTP的网页

"ProxyRouter": false,                   // 代理路由（使用 Grimoire 语言）
"CompleteProxy": false,                 // 在此基础上，全权代理路由（废弃默认路由）
"EnableSpawnProcess": false,            // 允许使用 Grimoire 脚本运行程序
"GrMain": "script/main.gr",             // Grimoire 主文件
"GrHotReload": false,                   // 启用热重载函数“hotreload”
"GrAutoReload": false,                  // 每次刷新页面都会热重载，可以单独启用该选项

"NotFoundForCode": false                //以 404 状态码代替 404 网页

```
不懂别乱配置

### 默认路由：
会将
- index.html
- index.htm
- index
路由到 index.html上

如果开启了 `ExtraFileStorages`， 如遇同名文件，以默认的public文件夹下的文件为最高优先级。
其次按`ExtraFileStorages`中的顺序递减。

如果开启了`StaticStorages`，遇文件夹与文件同名，则优先文件，在此情况下如欲访问文件夹则需输入如`/index/`，而非`/index`。

### Grimoire
包含`event`的文件必须是`GrMain`中配置的路径(默认`script/main.gr`)暂不支持其它位置。
如未设置 `CompleteProxy` 则若自定义路由未给出输出，则仍顺延至默认路由。
若设置 `CompleteProxy` 且为设置 `NotFoundForCode` ，则渲染 `404 Not Found`。
Grimoire Github：https://github.com/Enalye/grimoire


#### TTWeb 给出的 Grimoire 基本函数列表
```dlang

// TTWEB版本
ttlib.addVariable("TTWEB_VERSION", grString, GrValue(VERSION));
// 将内容输出到网页
ttlib.addFunction(&router_write, "router_write", [grString]);
// 传输文件
ttlib.addFunction(&router_serve, "router_serve", [grString]);
// 设置状态码 例如 404
ttlib.addFunction(&router_status, "router_status", [grInt]);
// 读文件，输入文件名，输出内容（由于架构问题，不可以在 event router 中调用）
ttlib.addFunction(&read_file, "read_file", [grString], [grString]);
// 写文件（实验性）
ttlib.addFunction(&write_file, "write_file", [grString, grString], [grBool]);
// 向控制台输出日志，第一个参数为代表报错等级的数字，分别是：
// all = 1
// trace = 32
// info = 64
// warning = 96
// error = 128
// critical = 160
// fatal = 192
ttlib.addFunction(&console_log, "console_log", [grInt, grString]);
// 向控制台输出自定义信息
ttlib.addFunction(&console_print, "console_print", [grString]);
// 第一个参数：待匹配字符串， 第二个参数：正则表达式，返回所有匹配的子串
ttlib.addFunction(&regex_, "regex", [grString, grString], [grList(grString)]);
// 输出调试信息（html格式）
ttlib.addFunction(&dump_html, "dump_html", [], [grString]);
// 输出调试信息（一般格式）
ttlib.addFunction(&dump_string, "dump_string", [], [grString]);
// 获得时间信息
ttlib.addFunction(&get_sec, "get_sec", [], [grInt]);
ttlib.addFunction(&get_min, "get_min", [], [grInt]);
ttlib.addFunction(&get_hour, "get_hour", [], [grInt]);
ttlib.addFunction(&get_day, "get_day", [], [grInt]);
ttlib.addFunction(&get_month, "get_month", [], [grInt]);
ttlib.addFunction(&get_year, "get_year", [], [grInt]);
if (g_enableSpawnProcess) // 如果设置了 `EnableSpawnProcess`
{
    // 调用程序 (命令形式) 例如: "ldc2.exe --help"， 也可以是任意命令，使用系统默认 Shell
    ttlib.addFunction(&execute_shell, "execute_shell", [grString], [
        grInt, grString
    ]); // 返回值：程序返回值， 程序的命令行输出
}
if (g_grhotreload)
    //热重载，有重大BUG，仅供测试
    ttlib.addFunction(&hotreload, "hotreload");
```

## Grimoire-TTLib （Grimoire脚本的TinyTinyWeb库）

- `ref.gr` 配置文件
- `alias.gr` 一些底层方法的再封装
- `log.gr` 输出日志
- `markdown.gr` 极为简易的html内嵌markdown支持
- `process.gr` 进程相关的操作
- `page.gr` 一些对页面的处理操作
- `stream.gr` “流”的定义

#### 内容

- `ref.gr`
- `alias.gr`
    + 函数 `file(string)(string)` 输入：`ref.gr/templatepath` 文件夹中的文件名 输出：文件内容
    + 输出流 `out` 用法：`out << "hello";`或`(out << "hello,")<<"world;"`
- `log.gr`
    + 枚举 `LogLevel` 定义了日志级别，值：`all`、`trace`、`info`、`warning`、`error`、`critical`、`fatal`
    + 函数 `toInt(LogLevel)(int)` 将 枚举 `LogLevel` 的枚举值转换成 `console_log` 函数可识别的格式
    + 函数 `speedtest(f: func())` 接受传入一个函数，测试函数的运行时间并以日志形式输出至控制台
- `markdown.gr`
    + 函数 `compile(string)(stringstream)` 传入一个字符串，编译后以`stringstream`的形式输出编译结果
- `process.gr`
    + 函数 `runShell(string)(string)` 输入 Shell 命令，返回 该 Shell 命令的返回内容
    + 函数 `printShell(string)()` 输入 Shell 命令，返回内容直接通过日志输入至控制台
- `page.gr`
    + 函数 `defaultErrPage(bool)(string)` 以字符串形式返回默认错误页，参数为是否包含 Dump 信息
    + 函数 `getErrPage(int, string)(string)` 获得一个字符串形式的错误页，第一个输入为错误码，第二个输入为错误页模板（编译时会自动替换模板内的`<gre>`标签为错误码）
    + 函数 `writeDefaultErrPage(int)(string)` 输入错误码，输出默认错误页
    + 函数 `autoRefresh(float)` 让网页定时自动刷新，参数为自动刷新间隔时间，单位秒
    + 函数 `build(string, list<string>)(string)` 自动替换指定字符串中的`<gr>`与`$_gr`标签为第二个参数的对应值，必须严格遵从前者字符串内的顺序
    + 函数 `build(string, string)(string)` 如果一个网页只有一处需要被 函数 `build` 替换的地方，这个重载可以不用让第二个参数使用数组
- `stream.gr`
    + 类 `stringstream`
成员：<br>
@stringstream(string)(stringstream) 通过字符串构造流<br>
@stringstream(string, bool)(stringstream) 通过字符串构造 CRLF 行尾序列的流（如果第二个输入为true）<br>
var str: string; 原始字符串<br>
var linenum: int; 当前行号<br>
var line: string; 当前行<br>
var index: int; 当前行首索引<br>
var size: int; 字符串大小<br>
var EOF: bool; 流是否已经执行到底<br>
var isCRLF: bool; 字符串是否是 CRLF 行尾序列的<br>
    + 函数`inc(stringstream)(stringstream)`将流递进一行，返回新流<br>


#### Events
目前开放了四个Events，分别是 `router`（每次请求都会调用，必须传入string）、`start`（进程初始化时调用）、`init`（随配置文件加载启动）、`stop`（进程结束时调用）。

#### 热重载
两种方法，需要分别在配置文件中开启。
`reload`函数手动热重载和每次刷新界面自动热重载。
均不建议在生产环境中使用。

## Grimoire VSCode 插件
官方版本：Grimoire Language
我编写的非官方版本：Grimoire-Support-Unofficial （很差，别用）

## Grimoire Tiny Tiny Lib
下载页中有提到，一个Grimoire语言的Lib，这并非一个通用算法库，仅是本项目提供的接口的再包装。

# 外部程序：

### Frp
配合 TTWebEasyStart 使用
https://gofrp.org/docs/