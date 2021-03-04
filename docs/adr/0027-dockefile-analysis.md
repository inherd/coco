# 27. dockefile analysis

Date: 2021-03-02

## Status

2021-03-02 proposed

## Context

[Dive](https://github.com/wagoodman/dive)  A tool for exploring each layer in a docker image.

[Release: Static analysis for Dockerfile](https://deepsource.io/blog/release-dockerfile-static-analysis/)

**Security issues:**

 - Last user should not be `root`
 - Use only an allowed registry in the `FROM` image

**Bug risks:**

 - `COPY --from` should reference a previously defined `FROM` alias
 - Multiple `ENTRYPOINT` instructions detected
 - Multiple `CMD` instructions detected

**Performance issues:**

 - Use `COPY` instead of `ADD` for files and folders
 - Use `ADD` for extracting archives into an image
 - Delete the `apt-get` lists after installing something

## Decision

Decision here...

## Consequences

Consequences here...
