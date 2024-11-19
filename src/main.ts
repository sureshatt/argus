import { Channel, invoke } from "@tauri-apps/api/core";

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

type NetLogEvent = {
  log: string
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

    var defaultChecked = false; // the default selected interface
    interfaces.forEach(iface => {
      const tr = document.createElement('tr');

      if (!defaultChecked && iface.ipv4_address != "127.0.0.1/8") {
        tr.innerHTML = `
          <td><input type="radio" name="rowSelect" value="${iface.name}" checked></td>
          <td>${iface.name}</td>
          <td>${iface.mac}</td>
          <td>${iface.ipv4_address}</td>
        `;
        defaultChecked = true;
      } else {
        tr.innerHTML = `
          <td><input type="radio" name="rowSelect" value="${iface.name}"></td>
          <td>${iface.name}</td>
          <td>${iface.mac}</td>
          <td>${iface.ipv4_address}</td>
        `;
      }
      tbody.appendChild(tr);
    });


  } catch (error) {
    console.error('Error fetching network interfaces:', error);
  }
}

fetchNetworkInterfaces();

window.addEventListener("DOMContentLoaded", () => {

  document.querySelector('#interfacesTable')?.addEventListener('change', () => {
    const selectedRadio = document.querySelector('input[name="rowSelect"]:checked') as HTMLInputElement;

    if (selectedRadio) {
      console.log(`Selected Row ID: ${selectedRadio.value}`);

      const channel = new Channel<NetLogEvent>();
      invoke('listen_to_event', { interface: selectedRadio.value, channel: channel });

      channel.onmessage = (event) => {
        console.log(`got NetLogEvent ${event.log}`);
        const messagesDiv = document.getElementById('messages');
        const messageDiv = document.createElement('div');
        messageDiv.classList.add('message');
        messageDiv.textContent = event.log;
        messagesDiv?.appendChild(messageDiv);
      };

    } else {
      console.log('No row selected');
    }
  });
});
