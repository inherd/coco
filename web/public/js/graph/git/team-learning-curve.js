function renderLearningCurve(data) {
    let allGroup = data;
    let weekNum = data[0].data.length;
    let minCommitNum = 99999, maxCommitNum = 0;

    allGroup.forEach(d => {
        let minMax = d3.extent(d.data, d => d);
        if (minCommitNum > minMax[0]) {
            minCommitNum = minMax[0];
        }
        if (maxCommitNum < minMax[1]) {
            maxCommitNum = minMax[1];
        }
    });
    console.log(data);
    let processData = allGroup.map((d, i) => {
        return {
            name: d.name,
            commitNums: d.data
        };
    })

    let margin = {top: 10, right: 100, bottom: 30, left: 30},
        width = GraphConfig.width - margin.left - margin.right,
        height = GraphConfig.height - margin.top - margin.bottom;

    let myColor = d3.scaleOrdinal()
        .domain(data)
        .range(d3.schemeSet2);

    let svg = d3.select("#learning-curve")
        .append("svg")
        .attr("width", width + margin.left + margin.right)
        .attr("height", height + margin.top + margin.bottom)
        .append("g")
        .attr("transform", `translate(${margin.left}, ${margin.top})`);


    // create x and y axis
    let x = d3.scaleLinear()
        .domain([0, weekNum])
        .range([0, width]);
    svg.append("g")
        .attr("transform", `translate(0, ${height})`)
        .call(d3.axisBottom(x));

    let y = d3.scaleLinear()
        .domain([minCommitNum, maxCommitNum])
        .range([height , 0]);
    svg.append("g")
        .call(d3.axisLeft(y));

    // create lines
    let line = d3.line()
        .x(function (d, i) {
            return x(i + 1);
        })
        .y(function (d) {
            return y(d);
        });
    svg.selectAll("lines")
        .data(processData)
        .enter()
        .append("path")
        .attr("d", function (d) {
            return line(d.commitNums);
        })
        .attr("stroke", function (d) {
            return myColor(d.name);
        })
        .style("stroke-width", 4)
        .style("fill", "none");

    // create dots
    svg.selectAll("dots")
        .data(processData)
        .enter()
        .append('g')
        .style("fill", function (d) {
            return myColor(d.name);
        })
        .selectAll("points")
        .data(function (d) {
            return d.commitNums;
        })
        .enter()
        .append("circle")
        .attr("class", "curving-point")
        .attr("cursor", "pointer")
        .attr("cx", function (d, i) {
            return x(i + 1);
        } )
        .attr("cy", function (d) {
            return y(d);
        } )
        .attr("r", 5)
        .attr("stroke", "white");

    const tooltip = d3.select("#learning-curve").append("div")
        .attr("class", "curve-tooltip")
        .style("position", "absolute")
        .style("visibility", "visible")
        .text("I am the tooltip")

    d3.selectAll(".curving-point")
        .on("mouseover", function() {
            return tooltip.style("visibility", "visible");
        })
        .on("mousemove", function(event, d) {
            return tooltip.style("top", (event.pageY-10) + "px").style("left", (event.pageX + 10) + "px");
        })
        .on("mouseout", function() {
            return tooltip.style("visibility", "hidden");
        });

}
