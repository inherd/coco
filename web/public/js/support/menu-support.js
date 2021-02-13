let MenuSupport = {
  menuFactory: function (x, y, menuItems, data, svg, offsetOptions) {
    d3.select(".context-menu").remove();

    // Draw the menu
    svg.append('g')
      .attr('class', "context-menu")
      .attr('transform', function (d) {
        if (!!offsetOptions && offsetOptions.width) {
          return 'translate(' + offsetOptions.width + ',' + offsetOptions.height + ')'
        }
        return 'translate(0, 10)';
      })
      .selectAll('tmp')
      .data(menuItems).enter()
      .append('g')
      .attr('class', "menu-entry")
      .style({'cursor': 'pointer'});

    // Draw menu entries
    d3.selectAll(`.menu-entry`)
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

    d3.selectAll(`.menu-entry`)
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
        d3.select(".context-menu").remove();
      });
  },
  createContextMenu: function (event, d, menuItems, svg, offsetOptions) {
    MenuSupport.menuFactory(event.layerX, event.layerY, menuItems, d, svg, offsetOptions);
    event.preventDefault();
  },
  copyText: function (text) {
    let ta = document.createElement("textarea");
    ta.value = text;
    ta.style.position = 'absolute';
    ta.style.left = "-999999999px";
    document.body.appendChild(ta);
    ta.select();
    document.execCommand('copy');
    document.body.removeChild(ta);
  },
  defaultMenuItems: [
    {
      title: 'Copy Path',
      action: (d) => {
        MenuSupport.copyText(d.data.path);
      }
    },
    {
      title: 'Open In Idea (Todo)',
      action: (d) => {
        // todo: add identify idea projects support
        window.open("jetbrains://open?url=" + d.data.path);
      }
    }
  ]
}
