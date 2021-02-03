# 9. git http server

Date: 2021-02-03

## Status

2021-02-03 proposed

## Context

In the case, we need a HTTP Server for query different git information:

1. team summary
   - join team
   - commits in month
2. git changes relations
3. git branches summary
4. ...

## Decision

Build a HTTP server, thinking in HTTP Server

1. with GraphQL: [juniper](https://github.com/graphql-rust/juniper) 
2. with Sled: [https://github.com/spacejam/sled](https://github.com/spacejam/sled)
3. with SQLite: [https://github.com/rusqlite/rusqlite](https://github.com/rusqlite/rusqlite)

## Consequences

Consequences here...
