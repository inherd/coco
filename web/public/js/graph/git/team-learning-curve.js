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
    let processData = allGroup.map((d, i) => {
        return {
            name: d.name,
            email: d.email,
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
        .range([height, 0]);
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
        .attr("id", function (d, i) {
            d.index = i;
            return `line-${i}`
        })
        .attr("d", function (d) {
            return line(d.commitNums);
        })
        .attr("stroke", function (d) {
            return myColor(d.email);
        })
        .style("stroke-width", 4)
        .style("fill", "none")
        .on("mouseover", function (event, d) {
            this.parentNode.appendChild(this);
            const sel = d3.select(`#point-group-${d.index}`);
            let pointGroup = sel._groups[0][0];
            pointGroup.parentNode.appendChild(pointGroup);
        });

    // set tooltip
    let tooltip = d3.select("#learning-curve").append("div")
        .attr("class", "curve-tooltip")
        .style("position", "absolute")
        .style("white-space", "pre-line")
        .style("visibility", "hidden")
        .style("font-weight", "bold")
        .style("background", "#ffffff")
        .style("border-radius", "4px")
        .style("box-shadow", "0px 0px 4px #e5e5e5")
        .style("padding", "6px")
        .text("I am the tooltip");

    // create dots
    svg.selectAll("dots")
        .data(processData)
        .enter()
        .append('g')
        .style("fill", function (d) {
            return myColor(d.email);
        })
        .attr("id", function (d, i) {
            return `point-group-${i}`
        })
        .selectAll("points")
        .data(function (d) {
            return d.commitNums.map(res => {
                return {
                    name: d.name,
                    email: d.email,
                    commitNum: res
                }
            });
        })
        .enter()
        .append("circle")
        .attr("class", "curving-point")
        .attr("cursor", "pointer")
        .attr("cx", function (d, i) {
            return x(i + 1);
        })
        .attr("cy", function (d) {
            return y(d.commitNum);
        })
        .attr("r", 5)
        .attr("stroke", "white")
        .on("mouseover", function () {
            tooltip.style("visibility", "visible");
        })
        .on("mousemove", function (event, d) {
            tooltip.style("top", (event.pageY - 10) + "px").style("left", (event.pageX + 10) + "px");
            tooltip.text(`commit num: ${d.commitNum} \n author: ${d.name} \n email: ${d.email}`);
        })
        .on("mouseout", function () {
            tooltip.style("visibility", "hidden");
        });

}
