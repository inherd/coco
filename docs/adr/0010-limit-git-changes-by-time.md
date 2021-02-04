# 10. limit git changes by time

Date: 2021-02-04

## Status

2021-02-04 proposed

## Context

Summary of Git commits by projects without FileChange

| Project Name     | Project Commits | Time   | Times(ms)         |
|------------------|-----------------|--------|-------------------|
| Rust Regex       | 1078            | 3s     | 2919ms ~ 3012ms   |
| Lombok           | 3127            | 8s     | 8096ms ~ 8616ms   |
| Nginx            | 6805            | 32s    | 32468ms ~ 33967ms |
| Redis            | 10009           | 67s    | 65328ms ~ 71616ms |
| Spring Framework | 22133           | 706s   |                   |
| Graal            | 49026           | 1425s  |                   |
| Gradle           | 78711           | 4130s  |                   |

If one project had lot of projects will be slowly.

## Decision

We can support git commits number.

## Consequences

Consequences here...
