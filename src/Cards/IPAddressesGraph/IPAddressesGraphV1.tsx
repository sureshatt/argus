import { useEffect, useRef, useState } from "react";
import Card from "../../components/card/Card";
import CardBody from "../../components/card/CardBody";
import CardHeader from "../../components/card/CardHeader";
import CardTitle from "../../components/card/CardTitle";
import { useNetStore } from "../../stores/net.store";
import { Network } from "../../services/network";
import { ExtensionCategory, Graph, register } from "@antv/g6";
import { AlertData, ArpStat } from "../../types";
import { ReactNode } from "@antv/g6-extension-react";
import { Icon } from "@iconify/react/dist/iconify.js";
import Alert from "../../components/alert/Alert";
import { errors } from "../../errors";

register(ExtensionCategory.NODE, "react", ReactNode);

interface GraphNode<T> {
  id: string;
  data: T;
}

interface GraphEdge {
  id: string;
  source: string;
  target: string;
}

interface GraphData<T> {
  nodes: GraphNode<T>[];
  edges: GraphEdge[];
}

function IPAddressesGraph() {
  const containerRef = useRef<HTMLDivElement>(null);
  const currentInterface = useNetStore((state) => state.currentInterface);
  const [show, setShow] = useState<boolean>(true);

  const drawTopology = async (data: any) => {
    if (data) {
      containerRef.current!.innerHTML = "";
      const graph = new Graph({
        animation: false,
        data: data as any,
        container: containerRef.current!,
        autoFit: { type: "center" },
        node: {
          type: "react",
          style: {
            size: [40, 80],
            component: (d: GraphNode<ArpStat>) => <Node data={d} />,
          },
        },
        edge: {
          style: {
            label: false,
            labelBackground: true,
          },
          state: {
            active: {
              label: true,
            },
            inactive: {
              strokeOpacity: 0,
            },
          },
        },
        layout: {
          type: "d3-force",
          link: {
            distance: 50,
            strength: 1,
          },
          collide: { radius: 35 },
        },
        // behaviors: [
        //   {
        //     type: "drag-element-force",
        //     fixed: true,
        //     animation: false,
        //   },
        // ],
      });
      await graph.render();
    }
  };

  const initialNode: GraphNode<ArpStat> = {
    id: "center",
    data: {
      source_mac: "",
      source_ip: "",
    },
  };
  const createGraphData = (data: ArpStat[]) => {
    const nodes: GraphNode<ArpStat>[] = [initialNode];
    const edges: GraphEdge[] = [];

    data.map((d, index) => {
      nodes.push({ id: d.source_mac, data: d });
      edges.push({
        id: `${d.source_mac}-to-center`,
        source: d.source_mac,
        target: "center",
      });

      // nodes.push({ id: index.toString(), data: d });
      // edges.push({
      //   id: `${index}-to-center`,
      //   source: index.toString(),
      //   target: "center",
      // });
    });
    return { nodes, edges };
  };

  const fetchData = async () => {
    if (currentInterface) {
      const d = await Network.getArpIpStats(currentInterface);
      const graphData = createGraphData(d);
      await drawTopology(graphData);
    }
  };

  let listener = 0;
  useEffect(() => {
    //console.log("INTERFACE CHANGED RERENDER!!!");
    clearInterval(listener);
    (async () => {
      if (currentInterface) {
        setShow(true);
        fetchData();
        listener = setInterval(fetchData, 5000);
      } else if (show) {
        setShow(false);
      }
    })();
    return () => clearInterval(listener);
  }, [currentInterface]);

  return (
    <Card cls="w-full h-full">
      {!show ? (
        <CardBody>
          <Alert
            value={errors.no_interface_selected}
            title="IP Addresses (and Country) to Which Most 
Traffic is Sent"
          />
        </CardBody>
      ) : (
        <>
          <CardHeader>
            <CardTitle
              value="IP Addresses (and Country) to Which Most 
Traffic is Sent"
            />
          </CardHeader>
          <CardBody>
            <div className="size-full overflow-hidden">
              <div
                className="size-full overflow-hidden"
                ref={containerRef}
              ></div>
            </div>
          </CardBody>
        </>
      )}
    </Card>
  );
}

export default IPAddressesGraph;

interface NodeProps {
  data: GraphNode<ArpStat>;
}

function Node({ data }: NodeProps) {
  const nodeRef = useRef<HTMLDivElement>(null);
  // const [parentDim, setParentDim] = useState<{ w: number; h: number }>({
  //   h: 0,
  //   w: 0,
  // });

  // const [nodePos, setNodePos] = useState<{ x: number; y: number }>({
  //   x: 0,
  //   y: 0,
  // });

  const getIcon = () => {
    const icons = [
      "fluent:phone-32-filled",
      "streamline:computer-pc-desktop-solid",
      "solar:laptop-bold",
      "bi:tv-fill",
      "solar:wi-fi-router-minimalistic-bold",
    ];

    const rand = Math.round((Math.random() * 10) % 4);
    return icons[rand];
  };

  // useEffect(() => {
  //   if (nodeRef.current) {
  //     const parentEl = nodeRef.current.parentElement;
  //     if (parentEl) {
  //       const { width, height } = parentEl.getBoundingClientRect();

  //       setParentDim({
  //         w: width,
  //         h: height,
  //       });

  //       const { x, y } = nodeRef.current.getBoundingClientRect();
  //       setNodePos({
  //         x,
  //         y,
  //       });
  //     }
  //   }
  // }, []);

  return (
    <div className="relative size-full flex   flex-col" ref={nodeRef}>
      <div className="peer">
        {data.id == "center" ? (
          <div className="flex justify-center items-center size-16 rounded-full bg-light-green-700">
            <Icon icon="fontisto:earth" className="size-10 text-cyan-500" />
          </div>
        ) : (
          <div className="flex justify-center items-center size-8 rounded-full bg-light-green-700">
            <Icon icon={getIcon()} className="size-5 text-cyan-500" />
          </div>
        )}
      </div>
      {data.id != "center" ? (
        <div
          className={`absolute top-9 peer-hover:z-[9999]  hover:z-50 -translate-x-1/2 flex flex-col bg-light-green-700 w-fit rounded p-0.5 text-[10px] text-light-green leading-3`}
        >
          <p className="text-nowrap">{data.data.source_ip}</p>
          <p className="text-nowrap">{data.data.source_mac} (MAC)</p>
        </div>
      ) : null}
    </div>
  );
}
