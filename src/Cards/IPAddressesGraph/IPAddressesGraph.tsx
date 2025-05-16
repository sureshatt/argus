import { useEffect, useRef, useState } from "react";
import Card from "../../components/card/Card";
import CardBody from "../../components/card/CardBody";
import CardHeader from "../../components/card/CardHeader";
import CardTitle from "../../components/card/CardTitle";
import { useNetStore } from "../../stores/net.store";
import { ExtensionCategory, Graph, register } from "@antv/g6";
import { ArpStat, NetworkStat } from "../../types";
import { ReactNode } from "@antv/g6-extension-react";
import { Icon } from "@iconify/react/dist/iconify.js";
import Alert from "../../components/alert/Alert";
import { errors } from "../../errors";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

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


function IPAddressesGraph() {
  const containerRef = useRef<HTMLDivElement>(null);
  const graphRef = useRef<Graph | null>(null);
  const currentInterface = useNetStore((state) => state.currentInterface);
  const [show, setShow] = useState<boolean>(true);

  const drawTopology = async (data: any) => {
    if (!data) return;

    if (!graphRef.current) {
      // Initialize graph if it doesn't exist
      graphRef.current = new Graph({
        animation: true,
        data: data as any,
        container: containerRef.current!,
        behaviors: ['drag-canvas', 'zoom-canvas', 'drag-element'],
        autoFit: { type: "view" },
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
          preventOverlap: true,
        },
      });
      await graphRef.current.render();
    } else {
      // Update existing graph data
      let existingData = graphRef.current.getData();
      let existingNodes = existingData.nodes;
      let newNodes = data.nodes.filter((node: GraphNode<any>) =>
        existingNodes.every((existingNode) => existingNode.id !== node.id)
      );

      if (newNodes.length === 0) {
        return;
      }

      let existingEdges = existingData.edges;
      let newEdges = data.edges.filter((edge: GraphEdge) =>
        existingEdges.every((existingEdge) => existingEdge.id !== edge.id)
      );

      let updatedData = {
        nodes: newNodes,
        edges: newEdges,
      };

      graphRef.current.addData(updatedData);
      graphRef.current.layout();

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

    data.forEach((d) => {
      nodes.push({ id: d.source_mac, data: d });
      edges.push({
        id: `${d.source_mac}-to-center`,
        source: d.source_mac,
        target: "center",
      });
    });
    return { nodes, edges };
  };

  let unlisten: UnlistenFn;
  
  const fetchData = async () => {
    if (currentInterface) {
      unlisten = await listen("stats", async (e) => {
        let networkStat = e.payload as NetworkStat;
        let arp_stats = networkStat.arp_stats;
        if (arp_stats.length > 0) {
          const graphData = createGraphData(arp_stats);
          await drawTopology(graphData);
        }
      });
    }
  };

  useEffect(() => {
    (async () => {
      if (currentInterface) {
        setShow(true);
        await fetchData();
      } else if (show) {
        setShow(false);
        // Clean up graph when interface is unset
        if (graphRef.current) {
          graphRef.current.destroy();
          graphRef.current = null;
        }
      }
    })();
    return () => {
      if (unlisten) unlisten();
      // Clean up graph on unmount
      if (graphRef.current) {
        graphRef.current.destroy();
        graphRef.current = null;
      }
    };
  }, [currentInterface]);

  return (
    <Card cls="w-full h-full">
      {!show ? (
        <CardBody>
          <Alert
            value={errors.no_interface_selected}
            title="ARP Network Discovery"
          />
        </CardBody>
      ) : (
        <>
          <CardHeader>
            <CardTitle
              value="ARP Network Discovery"
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

  return (
    <div className="relative size-full flex flex-col" ref={nodeRef}>
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
          className={`absolute top-9 peer-hover:z-[9999] hover:z-50 -translate-x-1/2 flex flex-col bg-light-green-700 w-fit rounded p-0.5 text-[10px] text-light-green leading-3`}
        >
          <p className="text-nowrap">{data.data.source_ip}</p>
          <p className="text-nowrap">{data.data.source_mac} (MAC)</p>
        </div>
      ) : null}
    </div>
  );
}
