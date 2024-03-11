# 2.0.0-beta10
```lisp
(do
    (log (input (str "You\bcan\binput\bsomething:"))) ; 可以输入一些内容
    (repl :debug)) ; 打开 REPL
```
# 2.0.0-beta11
```lisp
(log "Hello, World!") ; 新的解释器，比以前更快，比以前更好扩展
```
# 2.0.0-beta12
```lisp
(do
    (space
        (set a "test")
        (fly a))
    (log a) ; OK，a 飞出来了
    (drop a)
    (log a)) ; Error，a 已经被删除了
```
# 2.0.0-beta13
```lisp
(do
    (let a "test")
    (log a) ; OK
    (space
        (snatch a)
        (log a) ; OK
    (log a))) ; Error，snatch语句夺走了 a
```
# 2.0.0-beta14
```lisp
(do
    [it-is-a-lint] ; 这是给VSCode插件或Debugger等周边工具看的，也可以作为给人看的一种简短的提示
    (log [javascript] {
        alert("Hello, Javascript");
    })
    (log [markdown] {
        # Hello Markdown
    })
    (log [lisp] {
        (log "Hello Lisp")
    }))
```
遗憾的是，暂且无法为这种写法开发高亮——开发它的代价很大，所以 lint 语法基本上是预留的，暂时没什么用。

代码块功能（准确的说法是“原始字符串”）是已经可以正常使用的，它会被正常解析为字符串。

lint 也可以给各种辅助工具做提示，它会被解释器忽略。