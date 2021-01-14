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
    - [ ] merge code from [coca](https://github.com/phodal/coca/tree/master/pkg)
    - [ ] git tag analysis
    - [x] git branch analysis
       - [x] branch history
    - [ ] git commit time analysis
       - [ ] working night count
 - [ ] cloc analysis
    - [ ] spike cloc tools
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
 - [ ] suggestion API
    - [ ] suggest ledge
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

License
---

[![Phodal's Idea](http://brand.phodal.com/shields/idea-small.svg)](http://ideas.phodal.com/)

@ 2020~2021 A [Phodal Huang](https://www.phodal.com)'s [Idea](http://github.com/phodal/ideas).  This code is distributed under the MPL license. See `LICENSE` in this directory.
