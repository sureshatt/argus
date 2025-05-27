import Card from "../../components/card/Card";
import CardBody from "../../components/card/CardBody";
import JsonView from "react18-json-view";
import "./style.css";
import { useNetStore } from "../../stores/net.store";
import Alert from "../../components/alert/Alert";
import { errors } from "../../errors";
import { useEffect, useState } from "react";
import { Packet } from "../../types";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

function LogsJsonViewer() {
  const [currentLog, setCurrentLog] = useState<Packet>();
  const [show, setShow] = useState(false);
  const selectedInterface = useNetStore((state) => state.currentInterface);
  const selectedLog = useNetStore((state) => state.selectedLog);
  const autoViewNewLog = useNetStore((state) => state.autoViewNewLog);

  useEffect(() => {
    let unlisten: UnlistenFn;

    (async () => {

        if (selectedInterface && autoViewNewLog) {
          unlisten = await listen("all_logs_event", (e) => {
            const log = e.payload as Packet;    
              setCurrentLog(log);
              setShow(true);
          });
        } else {
          setCurrentLog(undefined);
          setShow(false);
        }
    })();

    return () => {
      if (unlisten) unlisten();
    };
  }, [selectedInterface]);

  return (
    <Card cls="w-full h-full">
      {show ? (
        <CardBody>
          <div className="p-2 h-full overflow-auto custom-scrollbar">
            <JsonView
              src={autoViewNewLog ? currentLog : selectedLog}
              displayArrayIndex={false}
              ignoreLargeArray={true}
              collapseObjectsAfterLength={50}
            />
          </div>
        </CardBody>
      ) : (
        <CardBody>
          <Alert
            value={errors.no_interface_selected}
            title="Network Log details Viewer"
            orientation="h"
          />
        </CardBody>
      )}
    </Card>
  );
}

export default LogsJsonViewer;
