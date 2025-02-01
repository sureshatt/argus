# Tauri + Vanilla TS

This template should help get you started developing with Tauri in vanilla HTML, CSS and Typescript.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## How to run the code
1. `pnpm tauri dev` for development
2. `pnpm tauri build` for distribution binary 

## Storage
example packet storeage
```json
{
   "destination":"String(""ff:ff:ff:ff:ff:ff"")",
   "ethernet_type":"String(""Arp"")",
   "id":"Object"{
      "tb":"String(""logs"")",
      "id":"Object"{
         "String":"String(""qojjdu97g144le1dmj1f"")"
      }
   },
   "info":"String("""")",
   "interface":"String(""en0"")",
   "length":"String(""60"")",
   "npid":"String(""000222"")",
   "parent":"String(""000221"")",
   "payload":"Array"[
      Number(0),
      Number(36),
      Number(75),
      Number(3),
      Number(250),
      Number(192),
      Number(1),
   ],
   "protocol":"String(""Ethernet"")",
   "source":"String(""24:4b:03:fa:36:cb"")",
   "timestamp":"String(""1738410151204"")"
}