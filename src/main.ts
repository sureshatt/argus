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

async function fetchNetworkInterfaces() {
  try {
    const tbody = document.querySelector('#interfacesTable tbody');

    if (tbody == null) {
      console.error("could not find interfacesTable");
      return;
    }

    tbody.innerHTML = ''; // Clear previous rows

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

      tbody.appendChild(tr);
    });

  } catch (error) {
    console.error('Error fetching network interfaces:', error);
  }
}

fetchNetworkInterfaces();

window.addEventListener("DOMContentLoaded", () => { // run only after the page is loaded

  document.querySelector('#interfacesTable')?.addEventListener('change', () => { // run only after the table changed

    document.querySelectorAll<HTMLInputElement>('input[name="rowSelect"]').forEach((input) => {
      input.addEventListener("change", (e: Event) => {
        const target = e.target as HTMLInputElement;
        invoke("set_selection", { selection: target.value });
      });
    });

    listen<string>("update", (event) => {
      console.log(`got NetLogEvent ${event.payload}`);

      const messagesDiv = document.getElementById("messages");
      if (messagesDiv) {
        const messageDiv = document.createElement('div');
        messageDiv.classList.add('message');
        messageDiv.textContent = event.payload;
        
        messagesDiv?.appendChild(messageDiv);
      }
    });

    
  });
});

invoke("start_loop");

