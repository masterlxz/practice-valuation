import { useMemo, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useQuery } from "@tanstack/react-query";
import {
  flexRender,
  getCoreRowModel,
  useReactTable,
  type ColumnDef,
} from "@tanstack/react-table";
import type { AppError, ValuationModel } from "../types";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";

const MODEL_LABELS: Record<string, string> = {
  bazin: "Bazin",
  graham: "Graham",
  gordon: "Gordon / DDM",
  dcf: "DCF / FCFF",
  banks: "Banks (P/B)",
  rnav: "RNAV",
  projected_ceiling: "Projected Ceiling",
};

type View = { screen: "list" } | { screen: "detail"; ticker: string };

type TickerSummary = {
  ticker: string;
  count: number;
  latestModel: string;
  latestVerdict: string | null;
  latestUpdatedAt: string;
};

// `list_valuations` already orders by updated_at desc, so the first row seen
// per ticker while iterating is the latest one.
function summarizeByTicker(valuations: ValuationModel[]): TickerSummary[] {
  const summaries = new Map<string, TickerSummary>();
  for (const v of valuations) {
    const existing = summaries.get(v.ticker);
    if (existing) {
      existing.count += 1;
    } else {
      summaries.set(v.ticker, {
        ticker: v.ticker,
        count: 1,
        latestModel: v.model,
        latestVerdict: v.verdict,
        latestUpdatedAt: v.updated_at,
      });
    }
  }
  return [...summaries.values()];
}

function VerdictBadge({ verdict }: { verdict: string | null }) {
  if (!verdict) return <span className="text-muted-foreground">—</span>;

  return (
    <Badge
      className={
        verdict === "BARATO"
          ? "bg-green-100 text-green-800 dark:bg-green-950 dark:text-green-300"
          : "bg-red-100 text-red-800 dark:bg-red-950 dark:text-red-300"
      }
    >
      {verdict}
    </Badge>
  );
}

function DataTable<T>({
  columns,
  data,
  onRowClick,
}: {
  columns: ColumnDef<T, any>[];
  data: T[];
  onRowClick?: (row: T) => void;
}) {
  const table = useReactTable({
    data,
    columns,
    getCoreRowModel: getCoreRowModel(),
  });

  return (
    <Table>
      <TableHeader>
        {table.getHeaderGroups().map((headerGroup) => (
          <TableRow key={headerGroup.id}>
            {headerGroup.headers.map((header) => (
              <TableHead key={header.id}>
                {header.isPlaceholder
                  ? null
                  : flexRender(
                      header.column.columnDef.header,
                      header.getContext(),
                    )}
              </TableHead>
            ))}
          </TableRow>
        ))}
      </TableHeader>
      <TableBody>
        {table.getRowModel().rows.length === 0 && (
          <TableRow>
            <TableCell
              colSpan={columns.length}
              className="text-center text-muted-foreground"
            >
              No saved valuations yet.
            </TableCell>
          </TableRow>
        )}
        {table.getRowModel().rows.map((row) => (
          <TableRow
            key={row.id}
            onClick={() => onRowClick?.(row.original)}
            className={onRowClick ? "cursor-pointer" : undefined}
          >
            {row.getVisibleCells().map((cell) => (
              <TableCell key={cell.id}>
                {flexRender(cell.column.columnDef.cell, cell.getContext())}
              </TableCell>
            ))}
          </TableRow>
        ))}
      </TableBody>
    </Table>
  );
}

const LIST_COLUMNS: ColumnDef<TickerSummary, any>[] = [
  { accessorKey: "ticker", header: "Ticker" },
  { accessorKey: "count", header: "Calculations" },
  {
    accessorKey: "latestModel",
    header: "Latest model",
    cell: (info) => MODEL_LABELS[info.getValue<string>()] ?? info.getValue(),
  },
  {
    accessorKey: "latestVerdict",
    header: "Latest verdict",
    cell: (info) => <VerdictBadge verdict={info.getValue<string | null>()} />,
  },
  { accessorKey: "latestUpdatedAt", header: "Last updated" },
];

const DETAIL_COLUMNS: ColumnDef<ValuationModel, any>[] = [
  {
    accessorKey: "model",
    header: "Model",
    cell: (info) => MODEL_LABELS[info.getValue<string>()] ?? info.getValue(),
  },
  { accessorKey: "reference_year", header: "Reference year" },
  {
    accessorKey: "current_price",
    header: "Current price",
    cell: (info) => `R$ ${info.getValue<number>().toFixed(2)}`,
  },
  {
    accessorKey: "fair_price",
    header: "Fair price",
    cell: (info) => {
      const value = info.getValue<number | null>();
      return value === null ? "—" : `R$ ${value.toFixed(2)}`;
    },
  },
  {
    accessorKey: "safety_margin",
    header: "Safety margin",
    cell: (info) => {
      const value = info.getValue<number | null>();
      return value === null ? "—" : `${(value * 100).toFixed(1)}%`;
    },
  },
  {
    accessorKey: "verdict",
    header: "Verdict",
    cell: (info) => <VerdictBadge verdict={info.getValue<string | null>()} />,
  },
  { accessorKey: "updated_at", header: "Updated at" },
];

function SavedValuationsPanel() {
  const [view, setView] = useState<View>({ screen: "list" });

  const valuationsQuery = useQuery<ValuationModel[], AppError>({
    queryKey: ["valuations"],
    queryFn: () => invoke("list_valuations"),
  });

  const valuations = valuationsQuery.data ?? [];

  const tickerSummaries = useMemo(
    () => summarizeByTicker(valuations),
    [valuations],
  );

  if (view.screen === "detail") {
    const ticker = view.ticker;
    const tickerValuations = valuations.filter((v) => v.ticker === ticker);

    return (
      <Card>
        <CardHeader>
          <div className="flex items-center gap-3">
            <Button
              variant="outline"
              size="sm"
              onClick={() => setView({ screen: "list" })}
            >
              Back
            </Button>
            <CardTitle>{ticker} — saved calculations</CardTitle>
          </div>
        </CardHeader>
        <CardContent>
          <DataTable columns={DETAIL_COLUMNS} data={tickerValuations} />
        </CardContent>
      </Card>
    );
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle>Saved valuations</CardTitle>
      </CardHeader>
      <CardContent>
        {valuationsQuery.isError && (
          <p className="text-red-600">{valuationsQuery.error.message}</p>
        )}
        <DataTable
          columns={LIST_COLUMNS}
          data={tickerSummaries}
          onRowClick={(row) => setView({ screen: "detail", ticker: row.ticker })}
        />
      </CardContent>
    </Card>
  );
}

export default SavedValuationsPanel;
