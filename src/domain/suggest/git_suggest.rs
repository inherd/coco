use crate::app::analysis::commit_analysis::ShortCommit;
use crate::domain::git::coco_tag::CocoTag;

#[allow(dead_code)]
pub struct GitSuggest {
    tags: Vec<CocoTag>,
    commits: Vec<ShortCommit>,
}

impl GitSuggest {
    /// count git tag interval for insight of release
    /// also same to publish interval
    /// 平均发布间隔
    pub fn git_tag_interval(&self) {}

    /// find multiple long branches working in process
    /// it will show the continuous delivery issue
    /// 最长分支
    pub fn long_branch_count(&self) {}

    /// show the data of weekend works' hours
    /// it will show the detail of hours
    /// 周末编码时间
    pub fn commits_in_weekend(&self) {}

    /// the time for max commits in days
    /// 最有效率时间
    pub fn most_efficiency_time(&self) {}

    /// show the average team members commit time
    /// frequently member's change means project's not stable for business project
    /// more members stay in a long time, will help project stable.
    /// unstable member's change need more Rules
    /// 平均成员编码时间区别
    pub fn average_members_coding_time_range(&self) {}

    /// the most active commits means the busy date
    /// 最活跃时间
    pub fn most_active_commits_date_by_month(&self) {}
}
