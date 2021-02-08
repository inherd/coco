
let MenuHandle = {
  copyText: function (text) {
    let ta = document.createElement("textarea");
    ta.value = text;
    ta.style.position = 'absolute';
    ta.style.left = "-999999999px";
    document.body.appendChild(ta);
    ta.select();
    document.execCommand('copy');
    document.body.removeChild(ta);
  }
}

let MenuSupport = {
  menuFactory: function (x, y, menuItems, data, svgId, width, height) {
    d3.select(".contextMenu").remove();

    // Draw the menu
    d3.select(svgId)
      .append('g')
      .attr('class', "contextMenu")
      .attr('transform', 'translate(' + (width - 600) + ',-' + (height + 100) + ')')
      .selectAll('tmp')
      .data(menuItems).enter()
      .append('g').attr('class', "menuEntry")
      .style({'cursor': 'pointer'});

    // Draw menu entries
    d3.selectAll(`.menuEntry`)
      .append('rect')
      .attr('x', x)
      .attr('y', (d, i) => {
        return y + (i * 30);
      })
      .attr('rx', 2)
      .attr('width', 150)
      .attr('height', 30)
      .on('click', (event, d) => {
        d.action(data)
      });

    d3.selectAll(`.menuEntry`)
      .append('text')
      .text((d) => {
        return d.title;
      })
      .attr('x', x)
      .attr('y', (d, i) => {
        return y + (i * 30);
      })
      .attr('dy', 20)
      .attr('dx', 45)
      .on('click', (event, d) => {
        d.action(data)
      });

    // Other interactions
    d3.select('body')
      .on('click', () => {
        d3.select(".contextMenu").remove();
      });
  },
  createContextMenu: function (event, d, menuItems, width, height, svgId) {
    MenuSupport.menuFactory(event.pageX - width / 2, event.pageY - height / 1.5, menuItems, d, svgId, width, height);
    event.preventDefault();
  }
}
