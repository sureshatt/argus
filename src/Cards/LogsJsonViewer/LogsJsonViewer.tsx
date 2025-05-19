import Card from "../../components/card/Card";
import CardBody from "../../components/card/CardBody";
import JsonView from "react18-json-view";
import "./style.css";
import { useNetStore } from "../../stores/net.store";
import Alert from "../../components/alert/Alert";
import { errors } from "../../errors";

function LogsJsonViewer() {
  const selectedLog = useNetStore((state) => state.selectedLog);

  return (
    <Card cls="w-full h-full">
      {selectedLog ? (
        <CardBody>
          <div className="p-2 h-full overflow-auto custom-scrollbar">
            <JsonView
              src={selectedLog}
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
