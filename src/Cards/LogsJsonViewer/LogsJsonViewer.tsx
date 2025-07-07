import Card from "../../components/card/Card";
import CardBody from "../../components/card/CardBody";
import JsonView from "react18-json-view";
import "./style.css";
import { useNetStore } from "../../stores/net.store";
import Alert from "../../components/alert/Alert";
import { errors } from "../../errors";
import { useEffect, useState } from "react";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { error, info, warn } from "@tauri-apps/plugin-log";

function LogsJsonViewer() {
  const [currentLog, setCurrentLog] = useState<JSON>();
  const [show, setShow] = useState(false);
  const selectedInterface = useNetStore((state) => state.currentInterface);
  const selectedLog = useNetStore((state) => state.selectedLog);
  const autoViewNewLog = useNetStore((state) => state.autoViewNewLog);

  useEffect(() => {
    info("LogsJsonViewer triggered with interface change. Resetting current log");
    let unlisten: UnlistenFn;

    (async () => {
      if (selectedInterface && autoViewNewLog) {
        setCurrentLog(JSON.parse('{}'));

        try {
          unlisten = await listen("all_logs_event", (e) => {

            if (!e || !e.payload) {
              warn("No payload received in all_logs_event");
              return;
            }

            const log = e.payload as JSON;
            setCurrentLog(log);
            setShow(true);
          });
        } catch (err) {
          error("Error fetching network logs:" + String(err));
        }

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
