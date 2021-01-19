# 7. regex match commit message design

Date: 2021-01-19

## Status

2021-01-19 proposed

## Context

Gitlab Docs: [https://docs.gitlab.com/ee/push_rules/push_rules.html](https://docs.gitlab.com/ee/push_rules/push_rules.html)

Gitlab use RE2: [https://github.com/google/re2/wiki/Syntax](https://github.com/google/re2/wiki/Syntax)

Rust RE2

```yaml
commit:
  regex: (\d{4})-(\d{2})-(\d{2})
  rules:
    - year
    - month
    - day

```


## Decision

Decision here...

## Consequences

Consequences here...
