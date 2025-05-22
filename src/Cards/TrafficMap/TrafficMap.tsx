import Card from "../../components/card/Card";
import CardBody from "../../components/card/CardBody";
import { useNetStore } from "../../stores/net.store";
import CardHeader from "../../components/card/CardHeader";
import CardTitle from "../../components/card/CardTitle";
import { useEffect, useLayoutEffect, useState } from "react";
import * as am5 from "@amcharts/amcharts5";
import * as am5map from "@amcharts/amcharts5/map";
import am5themes_Animated from "@amcharts/amcharts5/themes/Animated";
import am5geodata_worldLow from "@amcharts/amcharts5-geodata/worldLow";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { CountryStat, NetworkStat } from "../../types";

function TrafficMap() {
  const currentInterface = useNetStore((state) => state.currentInterface);
  const [pointSeries, setPointSeries] = useState<am5map.MapPointSeries>();

  useLayoutEffect(() => {
    var root = am5.Root.new("chartdiv");
    if (root) {
      root.setThemes([am5themes_Animated.new(root)]);

      let chart = root.container.children.push(
        am5map.MapChart.new(root, {
          projection: am5map.geoMercator(), // https://www.amcharts.com/docs/v5/charts/map-chart/#Built_in_projections
          panX: "rotateX",
          panY: "translateY",
          zoomLevel: 1,
        })
      );

      let legend = chart.children.push(am5.Legend.new(root, {}));
      legend.data.setAll(chart.series.values);

      let polygonSeries = chart.series.push( // https://www.amcharts.com/docs/v5/charts/map-chart/#Available_series_types
        am5map.MapPolygonSeries.new(root, {
          geoJSON: am5geodata_worldLow,
          exclude: ["AQ"],
        })
      );

      polygonSeries.mapPolygons.template.setAll({
        stroke: am5.color(0xffffff),
        strokeWidth: 0,
        strokeOpacity: 0,
        fillOpacity: 0,
        fillPattern: am5.CirclePattern.new(root, {
          color: am5.color(0x3ae7ff),
          checkered: true,
          gap: 1,
          radius: 1.34,
        }),
      });

      var ps = chart.series.push(
        am5map.MapPointSeries.new(root, {
          polygonIdField: "country",
        })
      );

      ps.bullets.push(function () {
        // Create container for pin and flag
        var container = am5.Container.new(root, {
          centerX: am5.p50,
          centerY: am5.p100, // Anchor at bottom center
          dy: -15, // Adjust pin position
          tooltipText: "{count}",
        });

        // Create pin shape using SVG path
        var pin = am5.Graphics.new(root, {
          fill: am5.color("#FF8E94"),
          svgPath:
            "M52.2133 29.5902C52.2133 37.8149 44.1973 47.0348 38.5125 52.5392C35.3829 55.5695 30.5566 55.5695 27.4269 52.5392C21.7422 47.0348 13.7261 37.8149 13.7261 29.5902C13.7261 18.9427 22.3418 10.3112 32.9697 10.3112C43.5976 10.3112 52.2133 18.9427 52.2133 29.5902Z",
          width: 66,
          height: 72,
        });

        pin.adapters.add("fill", function () {
          return am5.color("#11E4A6");
        });

        // Create pin shape using SVG path
        var circle = am5.Graphics.new(root, {
          fill: am5.color("#1D4741"),
          svgPath: "M14 2 A12 12 0 1 1 14 34 A12 12 0 1 1 14 2 Z",
          width: 32,
          height: 32,
          centerX: -17,
          centerY: -13,
        });

        const flag = am5.Picture.new(root, {
          width: 18,
          height: 12,
          centerX: -25,
          centerY: -22,
        });

        // Add adapter to access data
        flag.adapters.add("src", function (src, target) {
          // Access data context through dataItem
          const dataContext = target.dataItem?.dataContext as any;
          if (dataContext?.country) {
            return `https://flagcdn.com/${dataContext.name}.svg`;
          }
          return src;
        });

        container.children.push(pin);
        container.children.push(circle);
        container.children.push(flag);

        return am5.Bullet.new(root, {
          sprite: container,
        });
      });
      setPointSeries(ps);
    }
    return () => {
      root.dispose();
    };
  }, []);

  const fetchData = async (country_stats: CountryStat[]) => {
    if (currentInterface && pointSeries) {

      console.log("country stats", country_stats);
      const items = [
        ...country_stats.map((obj) => ({
          country: obj.country,
          name: obj.country.toLowerCase(),
          count: obj.count,
          t: "inbound",
        }))
      ];
      pointSeries.data.clear();
      pointSeries.data.pushAll(items);
    }
  };

 let unlisten: UnlistenFn;
  useEffect(() => {
    (async () => {
      if (currentInterface && pointSeries) {

        unlisten = await listen("stats", (e) => {
        let networkStat = e.payload as NetworkStat;
        let country_stats = networkStat.country_stats;
        
        fetchData(country_stats);
      });
      }
    })();
    return () => {
      if (unlisten) unlisten();
    };
  }, [currentInterface]);

  return (
    <Card cls="w-full h-full">
      <CardHeader>
        <CardTitle value="Most Traffic Sent & Received" />
      </CardHeader>
      <CardBody>
        <div className="w-full h-[calc(100%-40px)] relative">
          <div id="chartdiv" className="size-full"></div>
        </div>
        <div className="flex justify-center items-center gap-8 py-2">
          <div className="flex gap-2 justify-center items-center text-[#B8D6D0] font-semibold text-sm">
            <div className="size-6 bg-[#11E4A6] rounded"></div>
            <div>Inbound</div>
          </div>
          <div className="flex gap-2 justify-center items-center text-[#B8D6D0] font-semibold text-sm">
            <div className="size-6 bg-[#FF6E76] rounded"></div>
            <div>Outbound</div>
          </div>
        </div>
      </CardBody>
    </Card>
  );
}

export default TrafficMap;
