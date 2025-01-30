import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

type NetIface = {
  name: String,
  mac: String,
  ipv4_address: String,
  ipv6_addresses: String[],
  is_up: boolean,
  is_running: boolean,
  is_loopback: boolean,
  is_broadcast: boolean,
  is_multicast: boolean,
  is_p2p: boolean
}

type NetworkLog = {
  npid: string,
  parent: string,
  timestamp: string,
  protocol: string,
  source: string,
  destination: string,
  length: string,
  info: string,
  interface: string,
}

function handleRowSelectChange(event: Event) {
  const target = event.target as HTMLInputElement;
  invoke("set_selection", { selection: target.value });
  invoke("dump", { selection: target.value });
}

async function handlePacketRowSelect(event: Event) {
  const target = event.target as HTMLInputElement;
  if (target.parentNode) {
    const packetId = (target.parentNode as HTMLElement).id;
    console.log('Packet row selected:', packetId);

    try {
      const data: Array<any> = await invoke('get_packet_data', { packetId });
      console.log('Packet data:', data);
    } catch (error) {
      console.error('Failed to fetch packet data:', error);
    }
  }
}


function filterColumn(columnIndex: number): void {
  const input = document.querySelectorAll('thead input')[columnIndex] as HTMLInputElement;
  if (!input) return;

  const filter = input.value.toUpperCase();
  const table = document.getElementById('filterTable') as HTMLTableElement;
  if (!table) return;

  const tbody = table.querySelector('tbody');
  if (!tbody) return;

  const rows = tbody.getElementsByTagName('tr');

  for (let i = 0; i < rows.length; i++) {
      const cell = rows[i].getElementsByTagName('td')[columnIndex];
      if (cell) {
          const textValue = cell.textContent || cell.innerText;
          rows[i].style.display = textValue.toUpperCase().indexOf(filter) > -1 ? '' : 'none';
      }
  }
}

(window as any).filterColumn = filterColumn;


async function fetchNetworkInterfaces() {
  try {
    const tbody = document.querySelector('#interfacesTable tbody');

    if (tbody == null) {
      console.error("could not find interfacesTable");
      return;
    }

    tbody.innerHTML = ''; // Clear previous rows

    // handling network interfaces table
    const interfaces: NetIface[] = await invoke('get_network_interfaces');
    console.log('Network Interfaces:', interfaces);

    interfaces.forEach(iface => {
      const tr = document.createElement('tr');
      tr.innerHTML = `
          <td><input type="radio" name="rowSelect" value="${iface.name}"></td>
          <td>${iface.name}</td>
          <td>${iface.mac}</td>
          <td>${iface.ipv4_address}</td>
        `;
      tr.addEventListener("change", handleRowSelectChange);
      tbody.appendChild(tr);
    });

  } catch (error) {
    console.error('Error fetching network interfaces:', error);
  }
}

fetchNetworkInterfaces();


listen<NetworkLog>("update", (event) => {

  const netlog: NetworkLog = event.payload;
  console.log("got NetLogEvent",netlog);
``
  const filterTable = document.getElementById("filterTableBody");
  if (filterTable) {
    const trElement = document.createElement('tr');
    trElement.innerHTML = `
      <td>${netlog.npid}</td>
      <td>${netlog.timestamp}</td>
      <td>${netlog.protocol}</td>
      <td>${netlog.source}</td>
      <td>${netlog.destination}</td>
      <td>${netlog.length}</td>
      <td>${netlog.info}</td>
      
    `;
    trElement.id=netlog.npid;
    trElement.addEventListener("click", handlePacketRowSelect);

    filterTable?.prepend(trElement);
  }
});
