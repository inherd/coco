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
  matches:
    - year
    - month
    - day
```

Conventional Commit:

```yaml
commit:
 regex: (?<type>build)(?<scope>(?:\([^()\r\n]*\)|\()?(?<breaking>!)?)(?<subject>:.*)?
```

```yaml
branch:
  regex: ^(feature|fix)\/(([a-z,A-Z]+))(-)(\d*)(:)([a-z,0–9])
  matches:
    - branch
    - tag
    - id
  samples: feature/JIR-124:test commit message
```

Samples:

```yaml
branch:
  regex: (^(develop|dev|master|revert-[a-z0-9-]+)|(feature/|bug/|hotfix/|release/)[a-z0-9-]+)
  matches:
    - branch
    - tag
    - id
  samples:
```

## Decision

Decision here...

## Consequences

Consequences here...
