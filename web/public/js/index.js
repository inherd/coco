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

  renderPacking(data["reports"], "#circle-packing")
  renderNestedTreemap(data["reports"], "#nested-treemap")
  renderCodeFlower(data["reports"], '#code-flower');
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

  renderTagsTimeline(data, "#tags-timeline-select", "#tags-timeline");
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
  renderCodeFrequency(commitByWeeks, "#code-frequency-select", "#code-frequency");
});

d3.json("data/struct.json").then(function (data) {
  visualizationStruct(data);
});

d3.json("data/git-file-history.json").then(function (data) {
  data.width = GraphConfig.width;

  let options = ["-", "circles", "normal"];
  d3.select("#file-history-select")
    .selectAll('myOptions')
    .data(options)
    .enter()
    .append('option')
    .text(d => d)
    .attr("value", d => d)

  d3.select("#file-history-select").on("change", function (d) {
    let isCircles = false;
    if (d.target.value === "-") {
      return;
    }
    if (d.target.value === "circles") {
      isCircles = true;
    }

    let layout = calculateCodeLayout(data, isCircles);
    d3.select("#file-explorer").html("");
    renderCodeExplorer(layout, '#file-explorer');
  })
});

d3.json("data/pipeline.json").then(function (data) {
  if (!!data) {
    let pipeline = [];
    let first_pipeline = data[0];
    for (let stage of first_pipeline.stages) {
      let jobs = [];
      for (let sub_stage of stage.sub_stages) {
        jobs.push({
          name: sub_stage.name,
          desc: sub_stage.jobs,
        });
      }
      if (jobs.length === 0) {
        jobs.push({
          name: "",
          desc: jobs,
        });
      }
      pipeline.push({
        name: stage.name,
        children: jobs,
      })
    }

    visualizationPipeline(pipeline, '#pipeline');
  }
});
