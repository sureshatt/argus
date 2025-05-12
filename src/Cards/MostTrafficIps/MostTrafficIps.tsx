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
import { BarCharData, IpStat, NetworkStat } from "../../types";
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
  indexAxis: "y" as any,
  scales: {
    x: {
      grid: {
        display: false, // Change X-axis grid color
      },
      ticks: {
        color: "#3AE7FF", // Change x-axis label color
      },
    },
    y: {
      grid: {
        color: "rgba(255,255,255,0.15)", // Change Y-axis grid color
      },
      ticks: {
        autoSkip: false,
        color: "#A2BEB8", // Change y-axis label color
        font: {
          size: 8, // Set the font size here
        },
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


function MostTrafficIps() {
  const [data, setData] = useState<BarCharData>({
    labels: [],
    datasets: [
      {
        label: "Most Traffic",
        data: [],
        backgroundColor: "#3AE7FF",
        borderRadius: { bottomRight: 10, topRight: 10 },
        barThickness: 30,
      },
    ],
  });

  const [show, setShow] = useState(false);

  const selInterface = useNetStore((state) => state.currentInterface);

  const generateDataset = (d: IpStat[], label: "in" | "out") => {
    const labels: string[] = [];
    const data: number[] = [];
    const colors: string[] = [];

    d.forEach((s) => {
      labels.push(s.ip);
      data.push(s.count);
      colors.push(label == "in" ? "#3AE7FF" : "#3A4AFF");
    });

    return { labels, data, colors };
  };

  const handleDataChange = (incoming: IpStat[], outcoming: IpStat[]) => {
    const ingress = generateDataset(incoming, "in");
    const egress = generateDataset(outcoming, "out");

    //console.log("ingress", ingress, "egress", egress);

    const labels = [...ingress.labels, ...egress.labels];
    const data = [...ingress.data, ...egress.data];
    const bg = [...ingress.colors, ...egress.colors];

    const newData = {
      labels,
      datasets: [
        {
          label: "Most Traffic",
          data,
          backgroundColor: bg,
          borderRadius: { bottomRight: 10, topRight: 10 },
          barThickness: 5,
        },
      ],
    };
    //console.log("Done Bar");
    setData(newData);
  };


  let unlisten: UnlistenFn;
  useEffect(() => {
    (async () => {
      if (selInterface) {
        setShow(true);
        unlisten = await listen("stats", (e) => {
                let networkStat = e.payload as NetworkStat;
                const ingress = networkStat.ingress_ip_stats;
                const egress = networkStat.egress_ip_stats;
                if (ingress.length > 0 && egress.length > 0) {
                  setShow(true);
                   handleDataChange(ingress, egress);
                }
              });
      } else setShow(false);
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
            <CardTitle value="Most Traffic Sent & Received" />
          </CardHeader>
          <CardBody>
            <div className="w-full h-[calc(100%-40px)] relative">
              <Bar options={options} data={data} />
            </div>
            <div className="flex justify-center items-center gap-8 py-2">
              <div className="flex gap-2 justify-center items-center text-[#B8D6D0] font-semibold text-sm">
                <div className="size-6 bg-cyan-500 rounded"></div>
                <div>Sent</div>
              </div>
              <div className="flex gap-2 justify-center items-center text-[#B8D6D0] font-semibold text-sm">
                <div className="size-6 bg-[#3A4AFF] rounded"></div>
                <div>Received</div>
              </div>
            </div>
          </CardBody>
        </>
      ) : (
        <CardBody>
          <Alert
            value={errors.no_interface_selected}
            title="Most Traffic Sent & Received"
          />
        </CardBody>
      )}
    </Card>
  );
}

export default MostTrafficIps;
