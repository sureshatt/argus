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
import { getDeviceTypeFromMac } from "../../utils/tools";
import { warn, info, error, trace } from '@tauri-apps/plugin-log';

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

    if (data.nodes.length < 1) {
      warn("No nodes found in ARP stats data, quitting.");
      return;
    }

    if (!graphRef.current) {
      // Initialize graph if it doesn't exist
      info("Initializing the ARP graph ");

      graphRef.current = new Graph({
        animation: true, // https://g6.antv.antgroup.com/en/manual/graph/option
        data: data as any,
        container: containerRef.current!,
        behaviors: ['drag-canvas', 'zoom-canvas', 'drag-element'],
        autoFit: { type: "center" },
        node: { // https://g6.antv.antgroup.com/en/manual/element/node/build-in/base-node#type
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
          type: "radial",
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

      if (!existingData) {
        warn("No existing data found in ARP graph, Skipping update.");
        return;
      }

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

  const center_arp_stat: ArpStat = {
    source_mac: currentInterface?.mac || "",
    source_ip: currentInterface?.ipv4_address?.split("/")[0] || "",
  };

  const initialNode: GraphNode<ArpStat> = {
    id: center_arp_stat.source_mac,
    data: center_arp_stat,
  };

  const createGraphData = (data: ArpStat[]) => {
    const nodes: GraphNode<ArpStat>[] = [initialNode];
    const edges: GraphEdge[] = [];

    if (!data || data.length < 1) {
      return { nodes, edges };
    }

    data.forEach((d) => {
      nodes.push({ id: d.source_mac, data: d });
      edges.push({
        id: `${d.source_mac}-to-${initialNode.id}`,
        source: d.source_mac,
        target: initialNode.id,
      });
    });
    return { nodes, edges };
  };

  const fetchData = async (arp_stats: ArpStat[]) => {
    const graphData = createGraphData(arp_stats);
    await drawTopology(graphData);
  };

  useEffect(() => {

    info("IPAddressesGraph triggered with interface change. Resetting ARP stats graph");
    graphRef.current?.clear();

    let unlisten: UnlistenFn;
    (async () => {

      if (currentInterface) {
        setShow(true);
        try {
          unlisten = await listen("stats", async (e) => {
            try {
              let networkStat = e.payload as NetworkStat;

              if (!networkStat || !networkStat.arp_stats) {
                warn("No Network stats received in stats event");
                return;
              }

              let arp_stats = networkStat.arp_stats;
              if (!arp_stats) { // size check should NOT be done. With size 0, initial node will be drawn
                trace("No ARP stats available");
                return;
              }

              fetchData(arp_stats);
            } catch (err) {
              warn("Error processing stats event: " + String(err));
            }
          });
        } catch (err) {
          error("Error listening for stats:" + String(err));
        }
      } else {
        setShow(false);
      }
    })();
    return () => {
      setShow(false);
      info("Cleaning up listener for stats event in IPAddressesGraph");
      if (unlisten) unlisten();
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

  const getIcon = (ip: string, mac: string, center: boolean) => {

    if (center) {
      return "material-symbols:laptop-mac-outline"; // Default icon for center node
    }

    if (ip.endsWith(".1") || ip.endsWith(".254")) {
      return "ic:baseline-router"; // Router icon for common gateway IPs
    }

    const type = getDeviceTypeFromMac(mac);

    if (type === "phone") {
      return "fluent:phone-32-filled"; // Phone icon
    } else if (type === "laptop") {
      return "solar:laptop-bold"; // Laptop icon
    } else if (type === "desktop") {
      return "streamline:computer-pc-desktop-solid"; // Desktop icon
    } else if (type === "tv") {
      return "solar:tv-bold-duotone"; // TV icon
    } else {
      return "ri:device-fill";
    }
  };

  if (data.id == "center") {
    return (
      <div className="relative size-full flex flex-col" ref={nodeRef}>
        <div className="peer">
          <div className="flex justify-center items-center size-16 rounded-full bg-light-green-700">
            <Icon icon={getIcon(data.data.source_ip, data.data.source_mac, true)} className="size-10 text-cyan-500" />
          </div>
        </div>
        <div
          className={`absolute top-10 peer-hover:z-[9999] hover:z-50 translate-x-1/2 flex flex-col bg-light-green-700 w-fit rounded p-0.5 text-[8px] text-light-green leading-3`}
        >
          <p className="text-nowrap">{data.data.source_ip}</p>
          <p className="text-nowrap">{data.data.source_mac} (MAC)</p>
        </div>
      </div>
    );
  } else {
    return (
      <div className="relative size-full flex flex-col" ref={nodeRef}>
        <div className="peer">
          <div className="flex justify-center items-center size-8 rounded-full bg-light-green-700">
            <Icon icon={getIcon(data.data.source_ip, data.data.source_mac, false)} className="size-5 text-cyan-500" />
          </div>
        </div>
        <div
          className={`absolute top-9 peer-hover:z-[9999] hover:z-50 -translate-x-1/2 flex flex-col bg-light-green-700 w-fit rounded p-0.5 text-[8px] text-light-green leading-3`}
        >
          <p className="text-nowrap">{data.data.source_ip}</p>
          <p className="text-nowrap">{data.data.source_mac} (MAC)</p>
        </div>
      </div>
    );
  }
}
