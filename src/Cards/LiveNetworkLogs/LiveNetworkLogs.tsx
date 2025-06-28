import { useState } from "react";
import Card from "../../components/card/Card";
import CardBody from "../../components/card/CardBody";
import CardHeader from "../../components/card/CardHeader";
import CardTitle from "../../components/card/CardTitle";
import DataTable from "./DataTable";

function LiveNetworkLogs() {
  const [show, setShow] = useState(false);
  return (
    <Card cls="w-full h-full">
      {show ? (
        <CardHeader>
          <CardTitle value="Live Network Logs" />
        </CardHeader>
      ) : null}
      <CardBody>
        <DataTable showTitle={setShow} />
      </CardBody>
    </Card>
  );
}

export default LiveNetworkLogs;
