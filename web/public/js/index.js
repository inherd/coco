if (!window.d3) {
  alert("please run just setup in your project");
}

d3.json("data/cloc.json").then(function (json) {
  var data;
  var maxlen = 0;
  for (let i = 0; i < json.length; i++) {
    if (json[i].reports.length > maxlen) {
      maxlen = json[i].reports.length;
      data = json[i];
    }
  }

  renderPacking(data["reports"])
  renderNestedTreemap(data["reports"])
});

d3.json("data/git.json").then(function (json) {
  let data = [];
  for (let datum of json) {
    data.push({
      name: datum.name,
      author: datum.author,
      start: datum.first_commit_date,
      end: datum.last_commit_date,
      commits: datum.commits,
    })
  }

  renderBranches(data)
});

d3.json("data/git-tags.json").then(function (json) {
  renderTagsTimeline(json);
});

d3.json("data/git-commits.json").then(function (data) {
  renderCommitsTree(data);

  renderMembersTimeline(commits_by_authors(data));

  renderHeatmapChart("#hour-heatmap", commits_by_hours(data));
  renderHeatmapChart("#hour-heatmap-half-year", commits_by_hours(data, {before_month: 6}));
  renderHeatmapChart("#hour-heatmap-three-month", commits_by_hours(data, {before_month: 3}));
  renderLearningCurve(commits_by_users_with_range(data, 30));

  renderTeamCommitCalendar(commit_by_days(data), "#commit-calendar");
  renderTeamFrequency(commit_by_weeks(data));
});

