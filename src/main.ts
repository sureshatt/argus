import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

let selected_netIface: NetIface = {
  name: "",
  mac: "",
  ipv4_address: "",
  ipv6_addresses: [],
  is_up: false,
  is_running: false,
  is_loopback: false,
  is_broadcast: false,
  is_multicast: false,
  is_p2p: false
};

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
};

function handleRowSelectChange(event: Event) {
  // clear packets table
  const filterTable = document.getElementById("filterTableBody");
  if (filterTable) {
    filterTable.innerHTML = ""; // Clear previous rows
  }

  const packetTableBody = document.querySelector('#packetTable tbody');
  if (packetTableBody) {
    packetTableBody.innerHTML = ""; // Clear previous rows
  }

  // clear stats table
  const statsTableBody = document.querySelector('#statsTable tbody');
  if (statsTableBody) {
    statsTableBody.innerHTML = ""; // Clear previous rows
  }

  const target = event.target as HTMLInputElement;
  console.log('Row selected:', target.dataset.value);

  selected_netIface = JSON.parse(target?.dataset.value || "{}") as NetIface;
  console.log('Selected interface:', selected_netIface);
  
  invoke("set_selection", { selection: target.value });
  invoke("dump", { selection: target.value });
  handleNetLogStats();
}

async function handlePacketRowSelect(event: Event) {
  const target = event.target as HTMLInputElement;
  if (target.parentNode) {
    const parentId = (target.parentNode as HTMLElement).getAttribute("data-parent");
    console.log('Packet row selected with parent:', parentId);

    const parentDiv = document.getElementById('right-section') as HTMLTableElement;
    if (!parentDiv) {
      console.error("Parent div 'right-section' not found.");
      return;
    }

    let selectedPacketDiv = document.getElementById("selectedPacket");
    if (!selectedPacketDiv) {
      selectedPacketDiv = document.createElement("div");
      selectedPacketDiv.id = "selectedPacket";
      parentDiv.appendChild(selectedPacketDiv);
    }

    // Clear any existing content in selectedPacketDiv
    selectedPacketDiv.innerHTML = "";

    const table = document.createElement("table");
    table.id = "packetTable";
    const tbody = document.createElement("tbody");
    table.appendChild(tbody);
    selectedPacketDiv.appendChild(table);

    try {
      const data: Array<any> = await invoke('get_packet_data', { parentId: parentId, netIface: selected_netIface.name });
      console.log('Packet data:', data);

      data.forEach((item) => {
        const row = document.createElement("tr");
        const cell = document.createElement("td");

        // Ensure "payload" is printed as a single-line array
        const formattedItem = {
          ...item,
          payload: `[${item.payload.join(", ")}]` // Convert array to a single-line string
        };

        // Convert to JSON string with indentation
        cell.textContent = JSON.stringify(formattedItem, null, 2);

        // Styling to maintain formatting
        cell.style.fontFamily = "monospace";
        cell.style.whiteSpace = "pre-wrap"; // Ensures formatted JSON wraps properly

        row.appendChild(cell);
        tbody.appendChild(row);
      });
    } catch (error) {
      console.error("Error fetching packet data:", error);
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
      console.log('iface:', iface);
      const tr = document.createElement('tr');
      tr.innerHTML = `
          <td><input type="radio" name="rowSelect" value="${iface.name}" data-value='${JSON.stringify(iface)}'></td>
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

function handleNetLogEvent(event: any) {
  const netlog = event.payload as Record<string, any>;

  const filterTable = document.getElementById("filterTableBody");
  if (filterTable && typeof netlog === "object" && netlog !== null) {
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

    if (netlog.npid) {
      trElement.id = netlog.npid;
    }

    if (netlog.parent) {
      trElement.setAttribute("data-parent", netlog.parent);
    }

    trElement.addEventListener("click", handlePacketRowSelect);

    filterTable?.prepend(trElement);
  }
}

async function handleNetLogStats() {
  const data: Array<any> = await invoke('get_protocol_stats', { netIface: selected_netIface.name });
  const tbody = document.querySelector('#statsTable tbody');

  if (!tbody) {
    console.error("Stats table not found.");
    return;
  }

  tbody.innerHTML = ''; // Clear previous rows

  data.forEach((item) => {
    const row = document.createElement("tr");
    const cell1 = document.createElement("td");
    const cell2 = document.createElement("td");

    // Styling to maintain formatting
    cell1.style.fontFamily = "monospace";
    cell1.style.whiteSpace = "pre-wrap"; 
    cell2.style.fontFamily = "monospace";
    cell2.style.whiteSpace = "pre-wrap"; 

    cell1.innerHTML = item.protocol;
    cell2.innerHTML = item.count;


    row.appendChild(cell1);
    row.appendChild(cell2);
    tbody.appendChild(row);
  });
}

listen("update", (event) => {

  handleNetLogEvent(event);
});

setInterval(handleNetLogStats, 10000);

