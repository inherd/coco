# Coco

[![Coco Build](https://github.com/inherd/coco/actions/workflows/build.yml/badge.svg)](https://github.com/inherd/coco/actions/workflows/build.yml)

> (aka coconut, juice), an automatic DevOps metrics analysis tool.

case studies: [Coco cases](https://inherd.github.io/cases/)

Online Demos:

 - [Element-Plus](https://inherd.org/cases/element-plus/)
 - [Nginx](https://inherd.org/cases/nginx/)
 - [Redis](https://inherd.org/cases/redis/)

OS: macOS, Windows, GNU/Linux

## Usage

0. install or download Coco components from release, and download ctags: [windows](https://github.com/universal-ctags/ctags-win32/releases)
1. use `coco init` to generate config file
2. config `coco.yml`
3. optional: use `coco plugins` to download plugins
4. run Coco
   - `coco`, gather data from source
   - `visual`, visualization of data
   - `suggest`, generate suggestion
5. download plugins (optional)
   - coco_container，container analysis
   - coco_pipeline，pipeline analysis
   - coco_struct，model analysis by ctags
   - coco_swagger，Swagger API Analysis

visual URL:

 - all:  [http://127.0.0.1:8000/](http://127.0.0.1:8000/)
 - explorer: [http://127.0.0.1:8000/explorer.html](http://127.0.0.1:8000/explorer.html)

Offline usage:

1. download `coco`, `visual` e``xec file
2. download `coco_plugins_xxxx` with platform
3. extract `coco_plugins_xxxx` to `plugins` dir 

### Coco Usage

```
coco 0.5.3

USAGE:
    coco [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --branches <branches>            With all branches [default: true]
        --commits <commits>              With all commits [default: true]
    -c, --config-file <config-file>      Config file .yml [default: coco.yml]
    -d, --debug <debug>                  Debug mode [default: false]
    -f, --file-history <file-history>    Scan file change list from git & cloc [default: false]
    -y, --git-years <git-years>          Set git commits scan years, default 1, [default: 1.0]
    -t, --tags <tags>                    With all tags [default: true]

SUBCOMMANDS:
    help       Prints this message or the help of the given subcommand(s)
    init       Create default coco.yml files
    plugins    Download plugins from GitHub
```

sample:

```
coco --file-history=true --git-years=3
```

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
  - name: struct
    config:
      - key: ctags                       # set location for macOS ctags path
        value: /usr/local/bin/ctags
        
git:
  local: true # true will use `git clone`, false will use libgit2
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
    - [x] project calendar view [https://observablehq.com/@d3/calendar-view](https://observablehq.com/@d3/calendar-view)
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
 - [x] cloc analysis
    - [x] spike cloc tools [Tokei](https://github.com/XAMPPRocky/tokei)
    - [x] history cloc changes
    - [x] commit cloc changes
 - [ ] framework analysis.
    - framework detector
    - [x] merge from [scie-detector](https://github.com/datum-lang/scie/tree/master/scie-detector)
    - [x] framework output
    - [ ] tech stack generate
    - [ ] cloud native
 - [ ] module analysis
    - [x] base framework for directory
       - [x] gitignore support
    - [x] code flower
    - [ ] include analysis
       - [x] code parser: [pest](https://github.com/pest-parser/pest)
       - [ ] languages support.
 - [x] team analysis
    - [x] join time & life time
       - 以加入时间开始度量平均提交：上手成本分析
       - 平均加入时长
       - 成员加入时间点
    - [x] member growth
    - [x] count system size & learning curve
 - [ ] commit analysis
    - [x] commit times analysis (hours)
    - [x] rule regex support in config
    - [ ] participle（分词）
    - [x] tags generate
 - [ ] suggestion API
    - [ ] document manage system
    - [ ] suggest to ledge
    - [ ] suggest to cases 
    - [ ] suggest to coco.server?
       - [ ] online suggest ?
       - [ ] link daily checking
    - [ ] architecture design rules
 - [ ] architecture 
    - [ ] tech stack version check (more than 3 years ?) 
       - [ ] Maven Center 
       - [ ] NPM Server 
       - [ ] Go Server 
 - [ ] ~~tools~~
    - [ ] ~~tools config identify~~
    - [ ] ~~tools suggest (identify old tools)~~
    - [ ] ~~cloud-native config~~
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
    - [ ] PUML for call chain
       - [ ] symbol design - IsA, Use-In-The-Interface, Uses-In-The-Implementation  
 - [ ] plugin
    - [ ] struct analysis
       - [x] ctags
       - [ ] visual
       - [x] uml
    - [x] swagger
    - [x] Jenkinsfile
    - [ ] ~~Dockerfile~~

Visual and Reporter

 - visual api
     - [x] static files server
        - [x] http server: [actix_web](https://github.com/actix/actix-web)
        - [x] static server: [Rust Embed](https://github.com/pyros2097/rust-embed)
     - [x] export assets
     - [x] cli prompt for projects
     - [ ] query JSON API
     - [ ] CLI JSON API
 - visual web
     - [x] spike d3.js code organization
     - [ ] typescript with frontend framework
        - [ ] use Deno ?
     - [x] architecture
        - [x] first demo
        - [x] code flower 1, [D3.js code flower](https://github.com/fzaninotto/CodeFlower)
        - [x] circle
     - [ ] git
        - [x] branch history
     - [x] commits in years/month
        - examples: [gilot](https://github.com/hirokidaichi/gilot) average committer in month
        - [ ] changes in years/month
           - [x] [Stacked Area Chart](https://observablehq.com/@d3/stacked-area-chart)
    - [ ] graph support for velocity
        - [x] code commits by daily
        - ~~PR times by daily~~
    - [ ] story velocity
        - [ ] commit message analysis
        - [ ] story spend days
    - [ ] advanced code flower, [Polyglot Code Explorer](https://blog.korny.info/2020/09/06/introducing-the-polyglot-code-explorer.html)
        - source code: https://github.com/kornysietsma/polyglot-code-explorer/tree/master/src
        - [x] [Weighted Voronoi diagram](https://en.wikipedia.org/wiki/Weighted_Voronoi_diagram)
        - [ ] Age since last change
        - [ ] Creation date
        - [ ] Unique changers
        - [ ] Temporal Coupling
 - [ ] reporter
     - [ ] framework
     - [ ] cloc
     - [ ] git
     - [ ] architecture

DevOps pipeline

 - [ ] Jenkinsfile of Coco's examples

Tech Debt Integration

 - [ ] Git 2
     - [ ] Code check with: [https://github.com/kornysietsma/polyglot-code-scanner](https://github.com/kornysietsma/polyglot-code-scanner)
 - [ ] Integration Sonarqube ?
     - [ ] [Our 10-Point Technical Debt Assessment](https://codeclimate.com/blog/10-point-technical-debt-assessment/)

## Documents

Refs: [Libgit2 Documents](https://github.com/libgit2/libgit2.github.com/blob/master/docs/guides/101-samples/index.md)

## FAQ

### How to install in macOS?

#### Step1

Download Coco to your Mac from [release](https://github.com/daolou/coco/releases)

#### Step2

Move it(Coco) to your preferred directory,such as:

```zsh
mkdir ~/.coco
mv ~/Downloads/coco_macos ~/.coco/
```

#### Step3

Set up an alias to make it easier to use,such as:

```zsh
export 'alias coco="~/.coco/coco_macos"' >> ~/.zshrc
```

#### Step4

Try to use execute it by alias `coco`:

```zsh
coco -h
```

If it show no permission,you can execute the following command:

```zsh
chmod -R u=rwx,g=rw,o=r ~/.coco/coco_macos
```

Then retry:

```zsh
coco -h
```

Good luck!

## Thanks

[![Jetbrains](docs/images/jetbrains.svg)](https://www.jetbrains.com/?from=coco)

License
---

ctags analysis based on [https://github.com/dalance/ptags](https://github.com/dalance/ptags) with MIT, see in [src](plugins/coco_struct_analysis/src)

ctags parser rewrite from Golang's [https://github.com/ruben2020/tags2uml](https://github.com/ruben2020/tags2uml) with Apache License.

@ 2020~2021 This code is distributed under the MIT license. See `LICENSE` in this directory.
