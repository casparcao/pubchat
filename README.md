# openchat

## Architecture

```mermaid
sequenceDiagram
    participant Client
    participant ConnServer as Connection Server
    participant SessionServer as Session Server

    Client->>ConnServer: send message
    ConnServer->>SessionServer: forward to session queue
    SessionServer->>SessionServer: query all users in session
    SessionServer->>ConnServer: send to each user's message queue
    ConnServer->>Client: deliver message to user
```

## TODO

1. 简化user服务，使用core
2. 完善消息传输逻辑

## License

Copyright (c) caohailong

This project is licensed under the MIT license ([LICENSE] or <http://opensource.org/licenses/MIT>)

[LICENSE]: ./LICENSE
