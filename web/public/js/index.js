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
  renderTimeInteractiveLine(commitByDays, '#commit-contributions', 'value');

  let lineData = commitByDays.filter(d => d.total_line > 0);
  renderTimeInteractiveLine(lineData, '#line-history', 'total_line');

  let commitByWeeks = commit_by_weeks(data);
  renderCodeFrequency(commitByWeeks);
});

d3.json("data/struct.json").then(function (data) {
  visualizationStruct(data);
});

d3.json("fake/pipeline.json").then(function (data) {
  let testdata = [
    {
      name: 'Initialize',
      children: [
        { name: 'Initialize:success' }
      ]
    },
    {
      name: 'Build', children: [
        { name: 'Pull code:success' },
        { name: 'Test:error' },
        { name: 'Build:current' }
      ]
    },
    {
      name: 'Deploy', children: [
        { name: 'QA:pending' },
        { name: 'UAT:processing' },
        { name: 'STAGING:processing' },
        { name: 'PROD:untouched' }
      ]
    },
    {
      name: 'Finish', children: [
        { name: 'Finish:untouched' }
      ]
    }
  ];

  if (!!testdata) {
    visualizationPipeline(testdata, '#pipeline');
  }
});
