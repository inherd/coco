# Coco

[![Coco Build](https://github.com/inherd/coco/actions/workflows/build.yml/badge.svg)](https://github.com/inherd/coco/actions/workflows/build.yml)

> (aka coconut, juice), an automatic DevOps metrics analysis tool.

Online Reporter Demo: [https://inherd.github.io/coco/web/](https://inherd.github.io/coco/web/)

Case Studies: [Coco cases](https://inherd.github.io/cases/)

Support OS: macOS, Windows, GNU/Linux

特性（features in Chinese）：

 - 可交互式架构分析
 - 团队/项目健康值分析
 - 框架检测与分析
 - 云原生成熟度分析
 - 多项目**并行**分析
 - 分支生命周期和可视化
 - 改进建议（在线）

## Usage

0. install or download Coco components from release
1. use `coco init` to generate config file
2. config `coco.yml`
3. optional: use `coco plugins` to download plugins
4. run Coco
   - `coco`, gather data from source
   - `visual`, visualization of data
   - `suggest`, generate suggestion

### coco.yml

#### 配置 (config in Chinese)

示例：

```yml
# 代码库
repos:
  - url: https://github.com/coco-rs/coco.fixtures
  - url: https://github.com/coco-rs/coco.fixtures2
  - url: .
    languages: [Rust]
  - url: https://github.com/datum-lang/scie
  - url: https://github.com/projectfluent/fluent-rs
    languages: [Rust, JavaScript]                     # set languages for struct analysis

plugins:
  - name: swagger
  - name: struct_analysis
    config:
      - key: ctags                       # set location for macOS ctags path
        value: /usr/local/bin/ctags
```

## Documents

### Development

See in [DEVELOPMENT.md](DEVELOPMENT.md)

### Online video

Bilibili: [研发效能分析工具 Coco 第一次线上讨论](https://www.bilibili.com/video/BV18K4y1W7jN/)

### Roadmap

#### analysis and reporter

analysis

 - git analysis
    - branch
    - changes
    - commits
 - cloc analysis
    - summary
    - file arch
 - framework analysis
 - architecture analysis
    - file/directory organization

reporter

 - html reporter
 - json output
 - query api?

#### suggest and case study

### Tech

 - AST Parser: `include_parser/mod.rs` 
 - Web Server: `light_server.rs`
 - CLI: `visual.rs`, `coco.rs`, `suggest.rs`

## Todo

 - [x] git analysis
    - [x] merge code from [coca](https://github.com/phodal/coca/tree/master/pkg)
    - [x] local repo support
    - [ ] project calendar view [https://observablehq.com/@d3/calendar-view](https://observablehq.com/@d3/calendar-view)
    - [x] change
       - [x] line change
       - [x] changed file
       - [x] commit number
    - [x] git branch analysis
       - [x] branch history
       - [x] branch visual. such as [https://app.gfc.io/github/nvie/gitflow](https://app.gfc.io/github/nvie/gitflow)
           - ahead vs behind [https://github.com/BenoitZugmeyer/git-branches-overview](https://github.com/BenoitZugmeyer/git-branches-overview)
    - [x] git commit time analysis
       - [x] storage all commits
       - [x] working night count
 - [ ] cloc analysis
    - [x] spike cloc tools [Tokei](https://github.com/XAMPPRocky/tokei)
    - [ ] history cloc changes
    - [x] commit cloc changes
 - [ ] framework analysis.
    - framework detector
    - [x] merge from [scie-detector](https://github.com/datum-lang/scie/tree/master/scie-detector)
    - [x] framework output
    - [ ] tech stack generate
    - [ ] cloud native
        - [ ] dockerfile analysis
 - [ ] module analysis
    - [x] base framework for directory
       - [x] gitignore support
    - [x] code flower
    - [ ] include analysis
       - [x] code parser: [pest](https://github.com/pest-parser/pest)
       - [ ] languages support.
 - [ ] team analysis
    - [x] join time & life time
       - 以加入时间开始度量平均提交：上手成本分析
       - 平均加入时长
       - 成员加入时间点
    - [ ] member growth
    - [x] count system size & learning curve
    - [ ] micro services size
 - [ ] commit analysis
    - [ ] commit times analysis (hours)
    - [ ] rule regex support in config
    - [ ] participle（分词）
    - [ ] tags generate
 - [ ] suggestion API
    - [ ] document manage system
    - [ ] suggest to ledge
    - [ ] suggest to coco.server ?
    - [ ] online suggest
       - [ ] link daily checking
    - [ ] cases collection
    - [ ] architecture design rules
 - [ ] architecture 
    - [ ] tech stack version check (more than 3 years ?) 
       - [ ] Maven Center 
       - [ ] NPM Server 
       - [ ] Go Server 
 - [ ] tools
    - [ ] tools config identify
    - [ ] tools suggest (identify old tools)
    - [ ] cloud-native config
 - [ ] third-party integration
    - [ ] jenkins api analysis
    - [ ] test coverage integration
 - [ ] case study
    - [x] homepage: [https://github.com/inherd/cases](https://github.com/inherd/cases)
    - [ ] auto clone and auto deploy
 - [ ] todo scan
    - [ ] merge from [coca](https://github.com/inherd/coca/blob/master/pkg/application/todo/astitodo/astitodo.go)
 - [x] multiple platform support
    - [x] macOS
    - [x] GNU/Linux 
    - [x] Windows
       - [x] fix tests
       - [x] make it works
 - [ ] C4 Model
    - [ ] graphviz call chain
       - [ ] symbol design - IsA, Use-In-The-Interface, Uses-In-The-Implementation  
 - [ ] plugin
    - [ ] struct analysis
       - [x] ctags
       - [x] visual
    - [ ] swagger
    - [ ] coverage

Visual and Reporter

 - visual api
     - [x] static files server
        - [x] http server: [actix_web](https://github.com/actix/actix-web)
        - [x] static server: [Rust Embed](https://github.com/pyros2097/rust-embed)
     - [x] export assets
     - [ ] cli prompt for projects
     - [ ] query JSON API
     - [ ] CLI JSON API
 - visual web
     - [ ] spike d3.js code organization
     - [ ] typescript with frontend framework
        - [ ] use deno ?
     - [ ] architecture
        - [x] first demo
        - [ ] code flower, examples: [Polyglot Code Explorer](https://blog.korny.info/2020/09/06/introducing-the-polyglot-code-explorer.html), [D3.js code flower](https://github.com/fzaninotto/CodeFlower)
     - [ ] git
        - [x] branch history demo
     - [ ] commits in years/month
        - examples: [gilot](https://github.com/hirokidaichi/gilot) average committer in month
     - [ ] changes in years/month
        - [ ] [Stacked Area Chart](https://observablehq.com/@d3/stacked-area-chart)
    - [ ] graph support for velocity
        - [ ] code commits by daily
        - [ ] PR times by daily
    - [ ] story velocity
        - [ ] commit message analysis
        - [ ] story spend days
 - [ ] reporter
     - [ ] framework
     - [ ] cloc
     - [ ] git
     - [ ] architecture

DevOps pipeline

 - [ ] Jenkinsfile of Coco's examples

Tech Debt Integration

 - [ ] Integration Sonarqube ?
     - [ ] [Our 10-Point Technical Debt Assessment](https://codeclimate.com/blog/10-point-technical-debt-assessment/)

## Documents

Refs: [Libgit2 Documents](https://github.com/libgit2/libgit2.github.com/blob/master/docs/guides/101-samples/index.md)

## Thanks

[![Jetbrains](docs/images/jetbrains.svg)](https://www.jetbrains.com/?from=coco)

License
---

ctags analysis based on [https://github.com/dalance/ptags](https://github.com/dalance/ptags) with MIT, see in [src](plugins/coco_struct_analysis/src)

ctags parser rewrite from Golang's [https://github.com/ruben2020/tags2uml](https://github.com/ruben2020/tags2uml) with Apache License.

@ 2020~2021 This code is distributed under the MIT license. See `LICENSE` in this directory.
