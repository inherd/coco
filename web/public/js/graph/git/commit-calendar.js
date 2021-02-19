// based on https://observablehq.com/@d3/calendar-view
// Copyright 2018–2020 Observable, Inc.
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
function renderCommitCalendar(data, elementId = "#commit-calendar") {
  let weekday;

  let cellSize = 10;
  let width = GraphConfig.screen_width;
  let height = cellSize * (weekday === "weekday" ? 7 : 9)
  let timeWeek = weekday === "sunday" ? d3.utcSunday : d3.utcMonday;
  let countDay = weekday === "sunday" ? i => i : i => (i + 6) % 7;

  function pathMonth(t) {
    const n = weekday === "weekday" ? 5 : 7;
    const d = Math.max(0, Math.min(n, countDay(t.getUTCDay())));
    const w = timeWeek.count(d3.utcYear(t), t);
    return `${d === 0 ? `M${w * cellSize},0`
      : d === n ? `M${(w + 1) * cellSize},0`
        : `M${(w + 1) * cellSize},0V${d * cellSize}H${w * cellSize}`}V${n * cellSize}`;
  }

  let formatDay = i => "SMTWTFS"[i];
  let formatMonth = d3.utcFormat("%b");

  const max = d3.quantile(data, 0.9975, d => Math.abs(d.value));
  let color = d3.scaleLinear()
    .domain([0, +max])
    .range(["#9be9a8", "#216e39"])

  legend(
    {
      color,
      title: "Daily commits",
      ticks: 10,
      tickFormat: function (d) {
        return d;
      }
    },
    d3.select(elementId)
  )

  let years = d3.groups(data, d => d.date.getUTCFullYear()).reverse();

  const svg = d3.select(elementId).append("svg")
    .attr("viewBox", [0, 0, width, height * years.length])
    .attr("font-family", "sans-serif")
    .attr("font-size", 10);

  const year = svg.selectAll("g")
    .data(years)
    .join("g")
    .attr("transform", (d, i) => `translate(40.5,${height * i + cellSize * 1.5})`);

  year.append("text")
    .attr("x", -5)
    .attr("y", -5)
    .attr("font-weight", "bold")
    .attr("text-anchor", "end")
    .text(([key]) => key);

  year.append("g")
    .attr("text-anchor", "end")
    .selectAll("text")
    .data(weekday === "weekday" ? d3.range(1, 6) : d3.range(7))
    .join("text")
    .attr("x", -5)
    .attr("y", i => (countDay(i) + 0.5) * cellSize)
    .attr("dy", "0.31em")
    .text(formatDay);

  year.append("g")
    .selectAll("rect")
    .data(weekday === "weekday"
      ? ([, values]) => values.filter(d => ![0, 6].includes(d.date.getUTCDay()))
      : ([, values]) => values)
    .join("rect")
    .attr("width", cellSize - 1)
    .attr("height", cellSize - 1)
    .attr("x", d => timeWeek.count(d3.utcYear(d.date), d.date) * cellSize + 0.5)
    .attr("y", d => countDay(d.date.getUTCDay()) * cellSize + 0.5)
    .attr("rx", 2)
    .attr("ry", 2)
    .attr("fill", (d) => {
      if (d.value === 0) {
        return '#EBEDF0'
      }
      return color(d.value)
    })
    .append("title")
    .text(d => `date: ${standardFormatDate(d.date)}
commits: ${d.value}
${d.commits.map(commit => `${commit.author}: ${commit.message}`).join(`
`)}`);

  const month = year.append("g")
    .selectAll("g")
    .data(([, values]) => d3.utcMonths(d3.utcMonth(values[0].date), values[values.length - 1].date))
    .join("g");

  month.filter((d, i) => i).append("path")
    .attr("fill", "none")
    .attr("stroke", "#fff")
    .attr("stroke-width", 1)
    .attr("d", pathMonth);

  month.append("text")
    .attr("x", d => timeWeek.count(d3.utcYear(d), timeWeek.ceil(d)) * cellSize + 2)
    .attr("y", -5)
    .text(formatMonth);
}
