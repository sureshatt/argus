import { memo, useEffect, useMemo, useState } from "react";
import { Packet } from "../../types";
import { Icon } from "@iconify/react";
import {
  ColumnDef,
  flexRender,
  getCoreRowModel,
  useReactTable,
} from "@tanstack/react-table";
import { useNetStore } from "../../stores/net.store";
import { Network } from "../../services/network";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import CardBody from "../../components/card/CardBody";
import Alert from "../../components/alert/Alert";
import { errors } from "../../errors";

interface Props {
  showTitle: (value: boolean) => void;
}

function DataTable({ showTitle }: Props) {
  const [data, setData] = useState<Packet[]>(() => []);
  const [show, setShow] = useState(false);
  const MAX_ROWS = 100;

  const {
    currentInterface,
    setSelectedLog,
    autoViewNewLog,
    selectedLog,
    setAutoViewNewLog,
  } = useNetStore();

  const columns: ColumnDef<Packet>[] = useMemo(
    () => [
      {
        header: "",
        accessorKey: "chevron",
        cell: ({ row }) => {
          const id = row.original.npid;
          return (
            <div className="size-3 h-full flex justify-center items-center">
              <Icon
                icon="teenyicons:right-solid"
                className={`size-2 transition-all duration-300x ${
                  id === selectedLog?.npid ? "rotate-90 " : ""
                }`}
              />
            </div>
          );
        },
      },
      {
        header: "ID",
        accessorFn: (row: Packet) => row.npid,
      },
      {
        header: "TIMESTAMP",
        accessorFn: (row: Packet) => row.timestamp,
      },
      {
        header: "PROTOCOL",
        accessorFn: (row: Packet) => row.protocol,
      },
      {
        header: "SOURCE",
        accessorFn: (row: Packet) => row.source,
      },
      {
        header: "DESTINATION",
        accessorFn: (row: Packet) => row.destination,
      },
      {
        header: "LENGTH",
        accessorFn: (row: Packet) => row.length,
      },
      {
        header: "INFO",
        accessorFn: (row: Packet) => row.info,
        cell: ({ cell }) => {
          const value = cell.getValue() as string;
          return (
            <div
              title={value}
              className=" px-1.5  flex justify-center items-center"
            >
              {value ?? "-"}
            </div>
          );
        },
      },
    ],
    []
  );

  // Add this component inside DataTable.tsx
  const MemoizedRow = memo(
    ({
      row,
      isSelected,
      onClick,
    }: {
      row: any;
      isSelected: boolean;
      onClick: () => void;
    }) => (
      <tr
        className={`border-b-1 border-b-white/15 text-sm ${
          isSelected
            ? `text-[#EDEEEE] ${
                !autoViewNewLog ? "sticky bottom-1 bg-light-green-100" : ""
              }`
            : "text-light-green"
        }`}
        onClick={onClick}
      >
        {row.getVisibleCells().map((cell: any) => (
          <td key={cell.id} className="max-w-12 py-2 truncate">
            {flexRender(cell.column.columnDef.cell, cell.getContext())}
          </td>
        ))}
      </tr>
    ),
    // Custom comparison to prevent re-renders
    (prev, next) =>
      prev.row.original.npid === next.row.original.npid &&
      prev.isSelected === next.isSelected
  );

  const table = useReactTable({
    columns,
    data,
    getCoreRowModel: getCoreRowModel(),
    getRowId: (row) => row.npid.toString(),
  });

  useEffect(() => {
    let unlisten: UnlistenFn;
    (async () => {
      if (currentInterface) {
        setShow(true);
        showTitle(true);

        await Network.getNetworkLogs(currentInterface.name);
        unlisten = await listen("all_logs_event", (d) => {
          const log = d.payload as Packet;
          setData((prev) => {
            const updatedData = [log, ...prev];
            if (updatedData.length > MAX_ROWS) {
              updatedData.pop(); 
            }
            return updatedData;
          });
        });
      } else {
        setShow(false);
        showTitle(false);
      }
    })();

    return () => {
      if (unlisten) unlisten();
    };
  }, [currentInterface]);

  return show ? (
    <div className="w-full  font-quantic">
      <table className="w-full">
        <thead className="h-8 bg-linear-to-r from-[#23413E] to-[#0A3833] text-light-green-100 text-sm">
          {table.getHeaderGroups().map((headerGroup) => (
            <tr key={headerGroup.id} className="max-w-12">
              {headerGroup.headers.map((header) => (
                <th key={header.id} className="text-left">
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

        <tbody className="relative">
          {table.getRowModel().rows.map((row) => (
            <MemoizedRow
              key={row.id}
              row={row}
              isSelected={row.original.npid === selectedLog?.npid}
              onClick={() => {
                if (selectedLog?.npid === row.original.npid) {
                  setAutoViewNewLog(true);
                  return;
                }
                setAutoViewNewLog(false);
                setSelectedLog(row.original); // TODO: Get the full log details
              }}
            />
          ))}
        </tbody>
      </table>
    </div>
  ) : (
    <CardBody>
      <Alert value={errors.no_interface_selected} title="Live Network Logs" />
    </CardBody>
  );
}

export default DataTable;
