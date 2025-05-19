import Card from "../../components/card/Card";
import CardBody from "../../components/card/CardBody";
import CardHeader from "../../components/card/CardHeader";
import CardTitle from "../../components/card/CardTitle";
import DataTable from "./DataTable";

function AvailableNetworkInterface() {
  return (
    <Card cls="w-full h-full">
      <CardHeader>
        <CardTitle value="Available Network Interfaces" />
      </CardHeader>
      <CardBody>
        <DataTable />
      </CardBody>
    </Card>
  );
}

export default AvailableNetworkInterface;
