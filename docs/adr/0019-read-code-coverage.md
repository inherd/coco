# 19. read code coverage

Date: 2021-02-14

## Status

2021-02-14 proposed

## Context

Code coverage is a way for fitness, we need to collect test information in our projects. 

 - GCC or LLVM/Clang, with `.profraw` `.gcda` file
 - JavaScript with `.lcov` file
 - Java with `JaCoCo`

## Decision

1. Use [Grcov](https://github.com/mozilla/grcov) to collects and aggregates code coverage information.
2. Try integration it, if size it big, split as plugins. 

## Consequences

Consequences here...
