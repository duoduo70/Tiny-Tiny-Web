# Tiny Tiny Web
A project for Dlang. It can help you create a web server easily.

[docs(Chinese)](https://duoduo70.github.io/Tiny-Tiny-Web/)


### Prospect:

##### Now:

```mermaid
graph TD
subgraph Easy Start
E[Easy Start]
F[FRP]
E --> F
end
subgraph TTWeb
E --> M
M["main (If subthread started, automatic kill itself)"]
M --> A
M --> B
M --> C
M --> D
subgraph process1
A[Serverino] .- AA[GR-VM]
end
subgraph process2
B[Serverino] .- BA[GR-VM]
end
subgraph process3
C[Serverino] .- CA[GR-VM]
end
subgraph process4
D[Serverino] .- DA[GR-VM]
end
end
```

##### Future:

```mermaid
graph TD
subgraph Easy Start
E[Easy Start]
F[FRP]
E --> F
end
subgraph TTWeb
E --> M
M["main (can read/write stdstream and write to subprocess)"]
M .- A
M .- B
M .- C
M .- D
M .- GR
subgraph process_gr_vm
GR[GR-VM]
end
subgraph process1
A[Serverino] .- GR
end
subgraph process2
B[Serverino] .- GR
end
subgraph process3
C[Serverino] .- GR
end
subgraph process4
D[Serverino] .- GR
end
end
```

### Upstream

[serverino](https://github.com/trikko/serverino)

[grimoire](https://github.com/Enalye/grimoire)