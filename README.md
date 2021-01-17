# Coco

![Coco Build](https://github.com/phodal/coco/workflows/Coco%20Build/badge.svg)

> (aka coconut, juice), a automatic DevOps metrics tools.

特性（features in Chinese）：

 - 改进建议（英语）
 - 框架检测
 - 分支生命周期和可视化
 - 云原生成熟度分析
 - 团队健康值分析
 - 图形可视化
 - 多项目**并行**分析

features:

 - automatic suggestion (online).
 - framework detector
 - branch lifecycle and visual
 - cloud-native analysis
 - team health analysis
 - graph visual and reporter
 - multiple-repo **parallel**

## Todo

 - [x] git analysis
    - [x] merge code from [coca](https://github.com/phodal/coca/tree/master/pkg)
 - [ ] git tag analysis
    - [x] git branch analysis
       - [x] branch history
    - [ ] git commit time analysis
       - [ ] storage all commits
          - [ ] light database?
       - [ ] working night count
 - [ ] cloc analysis
    - [x] spike cloc tools [Tokei](https://github.com/XAMPPRocky/tokei)
    - [ ] history cloc changes
    - [ ] commit cloc changes
 - [ ] framework detector.
    - [ ] merge from [scie-detector](https://github.com/datum-lang/scie/tree/master/scie-detector)
 - [ ] module analysis
    - [ ] base framework for directory
    - [ ] code flower
 - [ ] team analysis
    - [ ] join time & life time
    - [ ] member growth
    - [ ] count system size & learning curve
 - [ ] commit analysis
    - [ ] rule regex support in config
    - [ ] participle（分词）
    - [ ] tags generate
 - [ ] suggestion API
    - [ ] suggest ledge
    - [ ] suggest phodal
    - [ ] online suggest
       - [ ] link daily checkx
 - [ ] graph support for velocity
    - [ ] code commits by daily
    - [ ] PR times by daily
 - [ ] tech stack generate
 - [ ] cloud native
    - [ ] dockerfile analysis
 - [ ] tools
    - [ ] tools config identify
    - [ ] tools suggest (identify old tools)
    - [ ] cloud-native config
 - [ ] case study
 - [ ] jenkins api analysis
 - [ ] story velocity
    - [ ] commit message analysis
    - [ ] story spend days

## Documents

Refs: [Libgit2 Documents](https://github.com/libgit2/libgit2.github.com/blob/master/docs/guides/101-samples/index.md)

License
---

[![Phodal's Idea](http://brand.phodal.com/shields/idea-small.svg)](http://ideas.phodal.com/)

@ 2020~2021 A [Phodal Huang](https://www.phodal.com)'s [Idea](http://github.com/phodal/ideas).  This code is distributed under the MIT license. See `LICENSE` in this directory.
