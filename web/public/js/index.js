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

d3.json("data/git-tags.json").then(function (data) {
  if (data.length <= 0) {
    throw Error("not tags");
  }

  renderTagsTimeline(data);
});

d3.json("data/git-commits.json").then(function (data) {
  d3.json("data/git.json").then(function (branches) {
    renderBranchTree(data, branches);
  });

  renderMembersTimeline(commits_by_authors(data));

  renderLearningCurve(commits_by_users_with_range(data, 30));

  renderHeatmapChart(commits_by_hours(data), "#hour-heatmap");
  renderHeatmapChart(commits_by_hours(data, {before_month: 6}), "#hour-heatmap-half-year");
  renderHeatmapChart(commits_by_hours(data, {before_month: 3}), "#hour-heatmap-three-month");

  let commitByDays = commit_by_days(data);

  renderCommitCalendar(commitByDays, "#commit-calendar");
  renderCommitContributions(commitByDays, '#commit-contributions');
  renderLineHistory(commitByDays, '#line-history');

  let commitByWeeks = commit_by_weeks(data);
  renderCodeFrequency(commitByWeeks);
});

d3.json("data/struct_analysis.json").then(function (data) {
  visualizationStruct(data);
});
