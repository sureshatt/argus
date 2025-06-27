import { useEffect, useState } from "react";
import Card from "../../components/card/Card";
import CardBody from "../../components/card/CardBody";
import CardHeader from "../../components/card/CardHeader";
import CardTitle from "../../components/card/CardTitle";

import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  BarElement,
  Title,
  Tooltip,
  Legend,
} from "chart.js";
import { Bar } from "react-chartjs-2";
import { useNetStore } from "../../stores/net.store";
import { BarCharData, NetworkStat, ProtocolStat } from "../../types";
import Alert from "../../components/alert/Alert";
import { errors } from "../../errors";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

ChartJS.register(
  CategoryScale,
  LinearScale,
  BarElement,
  Title,
  Tooltip,
  Legend
);

export const options = {
  responsive: true,
  maintainAspectRatio: false,
  scales: {
    x: {
      grid: {
        display: false, // Change X-axis grid color
      },
    },
    y: {
      grid: {
        color: "rgba(255,255,255,0.15)", // Change Y-axis grid color
      },
    },
  },
  plugins: {
    legend: {
      display: false,
      position: "top" as const,
    },
    title: {
      display: false,
    },
  },
};

const labels = ["Eth", "TCP", "IPv4", "UDP", "ARP"];

// const data =

const shadowPlugin = {
  id: "shadowPlugin",
  beforeDatasetsDraw(chart: any) {
    const { ctx } = chart;
    ctx.save();

    chart.data.datasets.forEach((_dataset: any, i: number) => {
      chart.getDatasetMeta(i).data.forEach((bar: any) => {
        ctx.shadowColor = "#3AE7FF36"; // Shadow color
        ctx.shadowBlur = 11; // Blur intensity
        ctx.shadowOffsetX = 0;
        ctx.shadowOffsetY = 4;
        ctx.fillRect(
          bar.x - bar.width / 2,
          bar.y + 10,
          bar.width,
          bar.height - 10
        );
        ctx.shadowColor = "#00000019"; // Shadow color
        ctx.shadowBlur = 4; // Blur intensity
        ctx.shadowOffsetX = 17;
        ctx.shadowOffsetY = 9;
        ctx.fillRect(
          bar.x - bar.width / 2,
          bar.y + 10,
          bar.width,
          bar.height - 10
        );
      });
    });

    ctx.restore();
  },
};

function MostFoundNetProtocol() {
  const [data, setData] = useState<BarCharData>({
    labels,
    datasets: [
      {
        label: "Dataset 1",
        data: [],
        backgroundColor: "#3AE7FF",
        borderRadius: { topLeft: 10, topRight: 10 },
      },
    ],
  });

  const [show, setShow] = useState(false);

  const selInterface = useNetStore((state) => state.currentInterface);

  const handleDataChange = (stats: ProtocolStat[]) => {
    const labels: string[] = [];
    const data: number[] = [];

    stats.forEach((s) => {
      labels.push(s.protocol);
      data.push(s.count);
    });

    const newData = {
      labels,
      datasets: [
        {
          label: "Protocols",
          data,
          backgroundColor: "#3AE7FF",
          borderRadius: { topLeft: 10, topRight: 10 },
        },
      ],
    };
    setData(newData);
  };


  useEffect(() => {
    // clear the chart when new interface is selected
    const stat: ProtocolStat[] = [];
    handleDataChange(stat);

    let unlisten: UnlistenFn;
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

  }, [selInterface]);

  return (
    <Card cls="w-full h-full">
      {show ? (
        <>
          <CardHeader>
            <CardTitle value="Network Protocols" />
          </CardHeader>
          <CardBody>
            <div className=" size-full relative">
              <Bar options={options} data={data} plugins={[shadowPlugin]} />
            </div>
          </CardBody>
        </>
      ) : (
        <CardBody>
          <Alert
            value={errors.no_interface_selected}
            title="Network Protocols"
          />
        </CardBody>
      )}
    </Card>
  );
}

export default MostFoundNetProtocol;
