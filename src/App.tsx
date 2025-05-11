import "./App.css";
import bg from "./assets/main-bg.png";
import AvailableNetworkInterface from "./Cards/AvailableNetworkInterfaces/AvailableNetworkInterface";
import IPAddressesGraph from "./Cards/IPAddressesGraph/IPAddressesGraph";
import LiveNetworkLogs from "./Cards/LiveNetworkLogs/LiveNetworkLogs";
import LogsJsonViewer from "./Cards/LogsJsonViewer/LogsJsonViewer";
import MostFoundNetProtocol from "./Cards/MostFoundNetProtocol/MostFoundNetProtocol";
import MostTrafficIps from "./Cards/MostTrafficIps/MostTrafficIps";
import TrafficMap from "./Cards/TrafficMap/TrafficMap";

function App() {
  return (
    <main className="relative w-screen h-screen overflow-hidden  bg-black bg-cover xbg-[center_top_1rem]">
      <img src={bg} alt="background" className="w-screen h-screen blur-sm" />
      <div className="absolute flex flex-col gap-3  top-0 left-0 w-screen h-screen bg-black/50 p-3">
        <div className="w-full h-1/2 flex gap-3">
          <div className="h-full w-2/3">
            <LiveNetworkLogs />
          </div>
          <div className="w-1/3 h-full flex flex-col  gap-2">
            <div className="w-full h-1/2">
              <AvailableNetworkInterface />
            </div>
            <div className="w-full h-1/2 overflow-hidden">
              <MostFoundNetProtocol />
            </div>
          </div>
        </div>
        <div className="h-[15%]">
          <LogsJsonViewer />
        </div>
        <div className="h-[35%] w-full flex gap-3 overflow-hidden">
          <div className="w-1/3 h-full">
            <IPAddressesGraph />
          </div>
          <div className="w-1/3 h-full">
            <MostTrafficIps />
          </div>
          <div className="w-1/3 h-full">
            <TrafficMap />
          </div>
        </div>
      </div>
    </main>
  );
}

export default App;
