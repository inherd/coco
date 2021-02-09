# Git Cli

## Commits per author

```
git shortlog -s -n
```

without merge

```
git shortlog -sn --no-merges
```

## Tags

list tags with refs

```
git show-ref --tags -d
```

with date

```
git log --tags --simplify-by-decoration --pretty="format:%t %at %d"
```
