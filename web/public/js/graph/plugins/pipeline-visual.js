/// MIT License
//
// Copyright (c) 2020 Ledge Framework @https://github.com/ledge-framework/engine
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

// based on https://github.com/ledge-framework/engine/blob/master/projects/%40ledge-framework/render/src/lib/chart/ledge-pipeline/ledge-pipeline.component.ts
function visualizationPipeline(data, elementId) {
  let Color = {
    GREEN: '#4A9900',
    RED: '#C4000A',
    GRAY: '#949393',
    WHITE: '#FFFFFF',
  }

  let PolygonPoints = {
    checkMark: '-2.00 2.80 -4.80 0.00 -5.73 0.933 -2.00 4.67 6.00 -3.33 5.07 -4.27',
    crossMark: '4.67 -3.73 3.73 -4.67 0 -0.94 -3.73 -4.67 -4.67 -3.73 -0.94 0 -4.67 3.73 -3.73 4.67 0 0.94 3.73 4.67 4.67 3.73 0.94 0',
  }

  let config = {
    connectionStrokeWidth: 4,
    stateStrokeWidth: 4,
    stateRadius: 8,
    stageSpace: 50,
    stageLabelHeight: 30,
    stageLabelSize: '12px',
    jobHeight: 50,
    jobLabelSize: '10px',
    startNodeRadius: 8,
    startNodeSpace: 30,
    endNodeRadius: 8,
    endNodeSpace: 30,
  };

  let stages = data.map(({name: stageName, children = []}) => ({
    label: stageName,
    jobs: children.map(({name: name}) => {
      let label = name;
      let state = 'success';
      return {label, state};
    })
  }));

  let svg = d3.select(elementId)
    .append('svg')
    .attr('width', '100%')
    .attr('viewBox', calculateViewBox(stages));

  renderPipeline(svg, stages);

  function renderPipeline(svg, stages) {
    const [startNode, ...rest] = generateTransformCoordinates(stages);
    const [endNode, ...reversedStages] = rest.reverse();

    renderStartNode(
      svg.append('g').attr('transform', `translate(${startNode.x}, ${startNode.y})`)
    );

    reversedStages.reverse().forEach(({x, y}, index) => {
      renderStage(
        svg.append('g').attr('transform', `translate(${x}, ${y})`),
        stages[index]
      );
    });

    renderEndNode(
      svg.append('g').attr('transform', `translate(${endNode.x}, ${endNode.y})`)
    );
  }

  /**
   * Each stage use transform do relative position
   * Composite Start/End node with stage nodes
   */
  function generateTransformCoordinates(stages) {
    const startNodeContainerWidth = 2 * config.startNodeRadius + config.startNodeSpace;
    const stageContainerWidth = 2 * (config.stageSpace + config.stateRadius);
    const yCoordinate = 60;
    const xCoordinates = [];
    // Start node
    const startCoordinates = [{x: 0, y: yCoordinate}];

    // Each stage
    stages.forEach((_, index) => {
      xCoordinates.push(stageContainerWidth * index);
    });

    // End node
    const endCoordinate = [{
      x: startNodeContainerWidth + stages.length * stageContainerWidth,
      y: yCoordinate,
    }];

    return [].concat(
      startCoordinates,
      xCoordinates.map(x => ({
        x: x + startNodeContainerWidth,
        y: yCoordinate,
      })),
      endCoordinate
    );
  }

  function renderStage(context, stage) {
    const [firstJob, ...restJobs] = stage.jobs;
    const isMultiJob = restJobs && !!restJobs.length;

    renderFirstJob(context, firstJob, isMultiJob);

    restJobs.forEach((job, jobIndex) => {
      renderJob(context, job, jobIndex + 1);
    });

    // Label
    context.append('text')
      .attr('x', config.stageSpace + config.stateRadius)
      .attr('y', -config.stageLabelHeight)
      .attr('text-anchor', 'middle')
      .attr('style', `font-size: ${config.stageLabelSize}`)
      .text(stage.label);
  }

  function renderFirstJob(context, job, isMultiJob) {
    // Line
    const path = d3.path();
    path.moveTo(0, 0);
    path.lineTo(2 * (config.stageSpace + config.stateRadius), 0);
    context.append('path')
      .attr('stroke-width', config.connectionStrokeWidth)
      .attr('stroke', Color.GRAY)
      .attr('d', path);

    // State
    renderState(
      context.append('g').attr('transform', `translate(${config.stageSpace}, 0)`),
      job.state
    );

    // Add label only for stage which has multiple jobs
    if (isMultiJob) {
      context.append('text')
        .attr('x', config.stageSpace + config.stateRadius)
        .attr('y', config.stageLabelHeight)
        .attr('text-anchor', 'middle')
        .attr('style', `font-size: ${config.jobLabelSize}`)
        .text(job.label);
    }
  }

  function renderJob(context, job, jobNumber) {
    renderJobLine(
      context.append('g')
        .attr('transform', `translate(0, -${config.stateRadius})`)
        .attr('fill', 'none')
        .attr('stroke-width', config.connectionStrokeWidth)
        .attr('stroke', Color.GRAY),
      jobNumber
    );

    // State
    renderState(
      context.append('g')
        .attr('transform', `translate(${config.stageSpace}, ${jobNumber * (2 * config.stateRadius + config.jobHeight)})`),
      job.state
    );

    // Label
    context.append('text')
      .attr('x', config.stageSpace + config.stateRadius)
      .attr('y', jobNumber * (2 * config.stateRadius + config.jobHeight) + config.stageLabelHeight)
      .attr('style', `font-size: ${config.jobLabelSize}`)
      .attr('text-anchor', 'middle')
      .text(job.label);
  }

  function renderJobLine(context, jobNumber) {
    const path = d3.path();
    const stateDiameter = 2 * config.stateRadius;
    path.arc(0, stateDiameter, config.stateRadius, Math.PI * 3 / 2, 0);
    path.arc(stateDiameter, jobNumber * (stateDiameter + config.jobHeight), config.stateRadius, Math.PI * 2 / 2, Math.PI * 1 / 2, true);
    path.arc(2 * config.stageSpace, jobNumber * (stateDiameter + config.jobHeight), config.stateRadius, Math.PI * 1 / 2, 0, true);
    path.arc(2 * (config.stageSpace + config.stateRadius), stateDiameter, config.stateRadius, Math.PI * 2 / 2, Math.PI * 3 / 2);
    context.append('path').attr('d', path);
  }

  function renderState(context, state) {
    const {
      polygonPoints,
      circleStrokeWidth,
      circleStroke,
      circleFill,
      symbolStroke,
      symbolFill,
    } = getStateConfig(state);

    // Circle
    const path = d3.path();
    path.arc(config.stateRadius, 0, config.stateRadius, 0, 2 * Math.PI);
    context.append('path')
      .attr('stroke-width', circleStrokeWidth)
      .attr('stroke', circleStroke)
      .attr('fill', circleFill)
      .attr('d', path);

    // Symbol
    context.append('g')
      .attr('transform', `translate(${config.stateRadius}, 0)`)
      .append('polygon')
      .attr('points', polygonPoints)
      .attr('stroke', symbolStroke)
      .attr('fill', symbolFill);
  }

  function renderStartNode(context) {
    // Circle
    const path = d3.path();
    path.arc(config.startNodeRadius, 0, config.startNodeRadius, 0, 2 * Math.PI);
    context.append('path').attr('d', path).attr('fill', Color.GRAY);

    // Connector
    const linePath = d3.path();
    linePath.moveTo(2 * config.startNodeRadius, 0);
    linePath.lineTo(2 * config.startNodeRadius + config.startNodeSpace, 0);
    linePath.closePath();
    context.append('path')
      .attr('d', linePath)
      .attr('stroke', Color.GRAY)
      .attr('stroke-width', config.connectionStrokeWidth);

    // Label
    context.append('text')
      .attr('x', config.startNodeRadius)
      .attr('y', -config.stageLabelHeight)
      .attr('text-anchor', 'middle')
      .text('Start');
  }

  function renderEndNode(context) {
    // Circle
    const path = d3.path();
    path.arc(config.endNodeSpace + config.endNodeRadius, 0, config.endNodeRadius, 0, 2 * Math.PI);
    context.append('path').attr('d', path).attr('fill', Color.GRAY);

    // Connector
    const linePath = d3.path();
    linePath.moveTo(0, 0);
    linePath.lineTo(config.endNodeSpace, 0);
    linePath.closePath();
    context.append('path')
      .attr('d', linePath)
      .attr('stroke', Color.GRAY)
      .attr('stroke-width', config.connectionStrokeWidth);

    // Label
    context.append('text')
      .attr('x', config.endNodeRadius + config.endNodeSpace)
      .attr('y', -config.stageLabelHeight)
      .attr('text-anchor', 'middle')
      .text('End');
  }


  function calculateViewBox(stages) {
    const maxJobsCountInStage = d3.max(stages.map(stage => stage.jobs.length));
    const startNodeWidth = 2 * (config.startNodeRadius + config.startNodeSpace);
    const stageWidth = 2 * (config.stageSpace + config.stateRadius + config.stateStrokeWidth);
    // Start/End node suppose to have same width
    const svgWidth = 2 * startNodeWidth + stages.length * stageWidth;

    const singleJobHeight = (2 * config.stateRadius + config.jobHeight);
    // With stage label height
    const svgHeight = config.stageLabelHeight + maxJobsCountInStage * singleJobHeight;

    // -20 for Start label, otherwise it will be cut off
    return `-20 0 ${svgWidth} ${svgHeight}`;
  }

  function getStateConfig(state) {
    const stateConfig = {
      polygonPoints: '',
      circleStrokeWidth: config.stateStrokeWidth,
      circleStroke: Color.WHITE,
      circleFill: Color.GREEN,
      symbolStrokeWidth: Color.WHITE,
      symbolStroke: Color.WHITE,
      symbolFill: Color.WHITE,
    };

    switch (state) {
      case 'success':
        return {
          ...stateConfig,
          polygonPoints: PolygonPoints.checkMark,
          circleFill: Color.GREEN,
          circleStroke: Color.GREEN,
        };
      case 'error':
        return {
          ...stateConfig,
          polygonPoints: PolygonPoints.crossMark,
          circleFill: Color.RED,
          circleStroke: Color.RED,
        };
      default:
        return {
          ...stateConfig,
          circleFill: Color.WHITE,
          circleStroke: Color.GRAY,
        };
    }
  }
}
