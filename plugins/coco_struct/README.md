# Struct Analysis

based on CTags

`cmd_ctags` based on [https://github.com/dalance/ptags](https://github.com/dalance/ptags)

## contribute new language

1. run analysis command

```
ctags --fields=+latinK -R code_path/datastore.go
```

2. copy generated `tags` to `coco/_fixtures/ctags`

3. write tests `in ctags_parser.rs`


## Todo

Todo:

 - [x] fix tests
 - [x] output data
 - [x] create data struct dsl
 - [x] summary data struct
 - [ ] data struct uml
    - [ ] [https://github.com/projectstorm/react-diagrams](https://github.com/projectstorm/react-diagrams) a super simple, no-nonsense diagramming library written in react that just works
    - [ ] JointJs [https://github.com/clientIO/joint](https://github.com/clientIO/joint) 
