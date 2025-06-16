import * as am5 from "@amcharts/amcharts5";
import * as am5xy from "@amcharts/amcharts5/xy";
import * as am5radar from "@amcharts/amcharts5/radar";
import am5themes_Animated from "@amcharts/amcharts5/themes/Animated";
import { useEffect, useLayoutEffect, useState } from "react";
import Card from "../../components/card/Card";
import CardBody from "../../components/card/CardBody";
import CardHeader from "../../components/card/CardHeader";
import CardTitle from "../../components/card/CardTitle";
import { useNetStore } from "../../stores/net.store";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { NetworkStat, ProtocolStat } from "../../types";

import { useRef } from "react";

//const rootRef = useRef<am5.Root | null>(null);

function MostFoundNetProtocol() {
  const currentInterface = useNetStore((state) => state.currentInterface);
  const [show, setShow] = useState(false);
  const chartRef = useRef<am5radar.RadarChart | null>(null);
  const seriesRef = useRef<am5radar.RadarColumnSeries | null>(null);



  useLayoutEffect(() => {
    var root = am5.Root.new("radarchartdiv");
    // rootRef.current = root;
    root.setThemes([am5themes_Animated.new(root)]);

    let chart = root.container.children.push(
      am5radar.RadarChart.new(root, {
        startAngle: 0,
        endAngle: 360,
        panX: false,
        panY: false,
      })
    );

    chartRef.current = chart;

    var xRenderer = am5radar.AxisRendererCircular.new(root, {});
    let xAxis = chart.xAxes.push(
      am5xy.CategoryAxis.new(root, {
        categoryField: "protocol",
        renderer: xRenderer
      })
    );

    var yRenderer = am5radar.AxisRendererRadial.new(root, {});
    let yAxis = chart.yAxes.push(
      am5xy.ValueAxis.new(root, {
        renderer: yRenderer
      })
    );

    yAxis.setAll({
      min: 0,
      strictMinMax: true,
      extraMax: 0.1 // optional, for padding
    });

    // Sample data
    var data = [
      { protocol: "TCP", count: 120 },
      { protocol: "UDP", count: 80 },
      { protocol: "ICMP", count: 40 },
      { protocol: "IP", count: 500 },
      { protocol: "ARP", count: 60 }
    ];

    //xAxis.data.setAll(data);

    let series = chart.series.push(
      am5radar.RadarColumnSeries.new(root, {
        name: "Protocol Count",
        xAxis: xAxis,
        yAxis: yAxis,
        valueYField: "count",
        valueXField: "protocol",
        tooltip: am5.Tooltip.new(root, {
          labelText: "{categoryX}: {valueY}"
        })
      })
    );

    series.columns.template.setAll({});

    //series.data.setAll(data);
    seriesRef.current = series;

    chart.set("cursor", am5radar.RadarCursor.new(root, {}));

    series.appear(1000);
    chart.appear(1000, 100);

    //let legend = chart.children.push(am5.Legend.new(root, {}));
    //legend.data.setAll(chart.series.values);

    return () => {
      root.dispose();
    };
  }, []);

  const handleDataChange = (stats: ProtocolStat[]) => {
    const chart = chartRef.current;
    const series = seriesRef.current;

    if (chart && series) {
      const data = stats.map((item) => ({
        protocol: item.protocol,
        count: item.count,
      }));

      chart.xAxes.getIndex(0)?.data.setAll(data); // update categories
      series.data.setAll(data); // update series values
      console.log("Series data:", series.data.values);
    }
  };


  let unlisten: UnlistenFn;
  useEffect(() => {
    (async () => {
      unlisten = await listen("stats", (e) => {
        let networkStat = e.payload as NetworkStat;
        let protocol_stats = networkStat.protocol_stats;
        if (protocol_stats.length > 0) {
          setShow(true);
          handleDataChange(protocol_stats);
        }
      });
    })();
    return () => {
      if (unlisten) unlisten();
    };

  }, [currentInterface]);

  return (
    <Card cls="w-full h-full">
      <CardHeader>
        <CardTitle value="Most Found Network Protocol" />
      </CardHeader>
      <CardBody>
        <div className=" size-full relative">
          <div id="radarchartdiv" className="size-full"></div>
        </div>
      </CardBody>
    </Card>
  );
}

export default MostFoundNetProtocol;
