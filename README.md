# openchat

(client) --send message--> (connection server) --send to session message queue--> (session server) --query all users in session, then send to user message queue--> (connection server) --send message to user--> (client)

## License

Copyright (c) caohailong

This project is licensed under the MIT license ([LICENSE] or <http://opensource.org/licenses/MIT>)

[LICENSE]: ./LICENSE
