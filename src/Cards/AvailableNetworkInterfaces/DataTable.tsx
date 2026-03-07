import { HTMLProps, useEffect, useRef, useState } from "react";
import { NetworkInterface } from "../../types";
import { warn, info, error } from '@tauri-apps/plugin-log';
import { listen } from "@tauri-apps/api/event";

import {
  ColumnDef,
  flexRender,
  getCoreRowModel,
  useReactTable,
} from "@tanstack/react-table";
import { Network } from "../../services/network";
import { useNetStore } from "../../stores/net.store";
import Alert from "../../components/alert/Alert";
import { errors } from "../../errors";

interface Props {}

function DataTable({}: Props) {
  const [data, setData] = useState<NetworkInterface[]>([]);
  const [permissionDenied, setPermissionDenied] = useState(false);

  const { setCurrentInterface, currentInterface } = useNetStore();

  const columns: ColumnDef<NetworkInterface>[] = [
    {
      header: "",
      accessorKey: "selected",
      cell: ({ row }) => {
        const name = row.original.name;
        return (
          <div className=" flex justify-center items-center">
            <IndeterminateCheckbox
              {...{
                checked: currentInterface?.name == name,
                onChange: (e) =>
                  (e.target as HTMLInputElement).checked
                    ? setCurrentInterface(row.original)
                    : undefined,
                className: "rounded bg-transparent checked:bg-light-green",
                name: "row-radio-selection",
              }}
            />
          </div>
        );
      },
    },
    {
      header: "Interf.",
      accessorFn: (row: NetworkInterface) => row.name,
    },
    {
      header: "MAC",
      accessorFn: (row: NetworkInterface) => row.mac,
    },
    {
      header: "IPv4",
      accessorFn: (row: NetworkInterface) => row.ipv4_address,
    },
    {
      header: "IPv6",
      accessorFn: (row: NetworkInterface) => row.ipv6_addresses,
    },
  ];

  const table = useReactTable({
    columns,
    data,
    getCoreRowModel: getCoreRowModel(),
  });

  useEffect(() => {
    (async () => {
      const hasPermission = await Network.checkCapturePermissions();
      if (!hasPermission) {
        error("BPF capture permission denied");
        setPermissionDenied(true);
        return;
      }
      const d = await Network.getAvailableNetworkInterfaces();
      if (d === undefined) {
        error("Failed to fetch network interfaces");
        return;
      }
      if (d.length === 0) {
        warn("No network interfaces found");
        return;
      }
      info(`Found ${d.length} network interfaces`);
      setData(d);
    })();
  }, []);

  useEffect(() => {
    const unlisten = listen("bpf_permission_error", () => {
      setPermissionDenied(true);
    });
    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  if (permissionDenied) {
    return (
      <Alert title="Permission Required" value={errors.bpf_permission_denied} />
    );
  }

  return (
    <div className="size-full font-quantic ">
      <table className="size-full table-auto border-collapse">
        <thead className="h-8 bg-linear-to-r from-[#23413E] to-[#0A3833] text-light-green-100 text-sm">
          {table.getHeaderGroups().map((headerGroup) => (
            <tr key={headerGroup.id}>
              {headerGroup.headers.map((header) => (
                <th key={header.id} className="text-left px-1">
                  {header.isPlaceholder
                    ? null
                    : flexRender(
                        header.column.columnDef.header,
                        header.getContext()
                      )}
                </th>
              ))}
            </tr>
          ))}
        </thead>
        <tbody>
          {table.getRowModel().rows.map((row) => (
            <tr
              key={row.id}
              className="border-b-1 border-b-white/15 max-w-full  text-light-green text-sm h-9"
            >
              {row.getVisibleCells().map((cell) => (
                <td
                  key={cell.id}
                  className={`break-all whitespace-normal px-1  `}
                >
                  {flexRender(cell.column.columnDef.cell, cell.getContext())}
                </td>
              ))}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

export default DataTable;

function IndeterminateCheckbox({
  indeterminate,
  className = "",
  ...rest
}: { indeterminate?: boolean } & HTMLProps<HTMLInputElement>) {
  const ref = useRef<HTMLInputElement>(null!);

  useEffect(() => {
    if (typeof indeterminate === "boolean") {
      ref.current.indeterminate = !rest.checked && indeterminate;
    }
  }, [ref, indeterminate]);

  return (
    <input
      type="radio"
      ref={ref}
      className={className + " cursor-pointer"}
      {...rest}
    />
  );
}
