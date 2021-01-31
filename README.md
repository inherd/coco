# Coco

![Coco Build](https://github.com/phodal/coco/workflows/Coco%20Build/badge.svg)

> (aka coconut, juice), an automatic DevOps metrics analysis tool.

Online Reporter Demo: [https://inherd.github.io/coco/web/](https://inherd.github.io/coco/web/)

Case Studies: [Coco cases](https://inherd.github.io/cases/)

特性（features in Chinese）：

 - 改进建议（英语）
 - 框架检测与分析
 - 云原生成熟度分析
 - 项目健康值分析
 - 图形可视化和报表生成
 - 多项目**并行**分析
 - 分支生命周期和可视化

features:

 - automatic suggestion (online).
 - framework detector and analysis
 - branch lifecycle and visual
 - cloud-native analysis
 - team health analysis
 - graph visual and reporter
 - multiple-repo **parallel**

## Usage

1. create `coco.yml` in projects.
2. config `coco.yml`
3. run `coco`

### coco.yml

#### 配置 (config in Chinese)

示例：

```yml
# 代码库
repo:
  - url: https://github.com/coco-rs/coco.fixtures
  - url: https://github.com/coco-rs/coco.fixtures2

# 提交信息格式
commit-message:
  # default: conventional commit: (?<type>build)(?<scope>(?:\([^()\r\n]*\)|\()?(?<breaking>!)?)(?<subject>:.*)?
  # jira: ^(feature|fix)\/(([a-z,A-Z]+))(-)(\d*)(:)([a-z,0–9])
  # jira test case: feature/JIR-124:test commit message
  regex: ^(feature|fix)\/(([a-z,A-Z]+))(-)(\d*)(:)([a-z,0–9])
  matches:
    - branch
    - tag
    - id
  samples: feature/JIR-124:test commit message
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
    - [x] git branch analysis
       - [x] branch history
       - [ ] branch visual. such as [https://app.gfc.io/github/nvie/gitflow](https://app.gfc.io/github/nvie/gitflow)
           - ahead vs behind [https://github.com/BenoitZugmeyer/git-branches-overview](https://github.com/BenoitZugmeyer/git-branches-overview)
    - [ ] git commit time analysis
       - [ ] storage all commits
          - [ ] light database?
          - [ ] light RESTful API?
       - [ ] working night count
 - [ ] cloc analysis
    - [x] spike cloc tools [Tokei](https://github.com/XAMPPRocky/tokei)
    - [ ] history cloc changes
    - [ ] commit cloc changes
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
    - [ ] code flower
    - [ ] include analysis
       - [x] code parser: [pest](https://github.com/pest-parser/pest)
       - [ ] languages support.
 - [ ] team analysis
    - [ ] join time & life time
    - [ ] member growth
    - [ ] count system size & learning curve
    - [ ] micro services size
 - [ ] commit analysis
    - [ ] rule regex support in config
    - [ ] participle（分词）
    - [ ] tags generate
 - [ ] suggestion API
    - [ ] suggest to ledge
    - [ ] suggest to coco.server ?
    - [ ] online suggest
       - [ ] link daily checking
 - [ ] tools
    - [ ] tools config identify
    - [ ] tools suggest (identify old tools)
    - [ ] cloud-native config
 - [ ] third-party integration
    - [ ] jenkins api analysis
    - [ ] test coverage integration
 - [ ] case study
 - [ ] todo scan
    - [ ] merge from [coca](https://github.com/inherd/coca/blob/master/pkg/application/todo/astitodo/astitodo.go)
 - [ ] multiple platform support
    - [x] macOS
    - [ ] GNU/Linux 
    - [ ] Windows
       - [ ] fix tests
       - [ ] make it works

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

[![PyCharm](docs/images/jetbrains.svg)](https://www.jetbrains.com/?from=coco)

License
---

@ 2020~2021 This code is distributed under the MIT license. See `LICENSE` in this directory.
