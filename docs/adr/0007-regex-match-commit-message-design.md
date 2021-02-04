# 7. regex match commit message design

Date: 2021-01-19

## Status

2021-01-19 proposed

2021-02-04 done

## Context

Gitlab Docs: [https://docs.gitlab.com/ee/push_rules/push_rules.html](https://docs.gitlab.com/ee/push_rules/push_rules.html)

Gitlab use RE2: [https://github.com/google/re2/wiki/Syntax](https://github.com/google/re2/wiki/Syntax)

Rust RE2

```yaml
commit-message:
  regex: (\d{4})-(\d{2})-(\d{2})
  matches:
    - year
    - month
    - day
```

Conventional Commit:

```yaml
commit-message:
 default: (?<type>build)(?<scope>(?:\([^()\r\n]*\)|\()?(?<breaking>!)?)(?<subject>:.*)?
```

Jira Samples

```yaml
commit-message:
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
