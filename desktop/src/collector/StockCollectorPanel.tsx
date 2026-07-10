import { invoke } from "@tauri-apps/api/core";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import type { AppError } from "../types";
import { Button } from "@/components/ui/button";
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

type CollectorSummary = {
  success: boolean;
  output: string;
};

type StockQuote = {
  id: number;
  ticker: string;
  price: number;
  source: string;
  fetched_at: string;
};

type StockFundamentals = {
  id: number;
  ticker: string;
  lpa: number;
  vpa: number;
  roe: number;
  source: string;
  fetched_at: string;
};

type StockDividendsAvg = {
  id: number;
  ticker: string;
  avg_dividend_5y: number;
  source: string;
  fetched_at: string;
};

type StockDcfFundamentals = {
  id: number;
  ticker: string;
  reference_year: number;
  ebit: number;
  depreciation_amortization: number | null;
  capex: number | null;
  nwc_change: number;
  total_debt: number;
  cash: number;
  shares_outstanding: number;
  source: string;
  fetched_at: string;
};

// `list_stock_*` commands order by fetched_at desc, so the first row seen
// per ticker while iterating is the latest one — same pattern used for
// crypto readings and saved valuations.
function latestPerTicker<T extends { ticker: string }>(rows: T[]): Map<string, T> {
  const latest = new Map<string, T>();
  for (const row of rows) {
    if (!latest.has(row.ticker)) {
      latest.set(row.ticker, row);
    }
  }
  return latest;
}

function formatDateTime(iso: string): string {
  return new Date(iso).toLocaleString(undefined, {
    year: "numeric",
    month: "short",
    day: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  });
}

function StockCollectorPanel() {
  const queryClient = useQueryClient();

  const quotesQuery = useQuery<StockQuote[], AppError>({
    queryKey: ["stock-quotes"],
    queryFn: () => invoke("list_stock_quotes"),
  });

  const fundamentalsQuery = useQuery<StockFundamentals[], AppError>({
    queryKey: ["stock-fundamentals"],
    queryFn: () => invoke("list_stock_fundamentals"),
  });

  const dividendsAvgQuery = useQuery<StockDividendsAvg[], AppError>({
    queryKey: ["stock-dividends-avg"],
    queryFn: () => invoke("list_stock_dividends_avg"),
  });

  const dcfFundamentalsQuery = useQuery<StockDcfFundamentals[], AppError>({
    queryKey: ["stock-dcf-fundamentals"],
    queryFn: () => invoke("list_stock_dcf_fundamentals"),
  });

  const runMutation = useMutation<CollectorSummary, AppError, void>({
    mutationFn: () => invoke("run_stock_collector"),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["stock-quotes"] });
      queryClient.invalidateQueries({ queryKey: ["stock-fundamentals"] });
      queryClient.invalidateQueries({ queryKey: ["stock-dividends-avg"] });
      queryClient.invalidateQueries({ queryKey: ["stock-dcf-fundamentals"] });
    },
  });

  const latest = [...latestPerTicker(quotesQuery.data ?? []).values()];
  const latestFundamentals = [
    ...latestPerTicker(fundamentalsQuery.data ?? []).values(),
  ];
  const latestDividendsAvg = [
    ...latestPerTicker(dividendsAvgQuery.data ?? []).values(),
  ];
  const latestDcfFundamentals = [
    ...latestPerTicker(dcfFundamentalsQuery.data ?? []).values(),
  ];

  return (
    <Card>
      <CardHeader>
        <CardTitle>Data Collector — Stocks</CardTitle>
      </CardHeader>
      <CardContent className="flex flex-col gap-4">
        <p className="text-sm text-muted-foreground">
          Runs the Python collector (brapi for quotes, bolsai for fundamentals
          and dividends, CVM's open data for DCF fundamentals) for the
          tickers configured in <code>data-collector/config.yaml</code> and
          saves each reading as a new row — nothing is overwritten. Bolsai
          requires <code>BOLSAI_API_KEY</code> in{" "}
          <code>data-collector/.env</code>; if it's missing, quotes still
          update and the tables below stay empty. DCF fundamentals depend on
          bolsai's fundamentals lookup, not on your own API key/signup for
          CVM (no key needed) — D&A/Capex show "—" when the account
          couldn't be matched confidently for that company (see
          <code> cvm_dfp.py</code>).
        </p>

        <Button
          onClick={() => runMutation.mutate()}
          disabled={runMutation.isPending}
          className="w-fit"
        >
          {runMutation.isPending ? "Running..." : "Run stock collector"}
        </Button>

        {runMutation.isSuccess && !runMutation.data.success && (
          <p className="whitespace-pre-wrap text-red-600">
            {runMutation.data.output}
          </p>
        )}
        {runMutation.isError && (
          <p className="text-red-600">{runMutation.error.message}</p>
        )}

        {quotesQuery.isError && (
          <p className="text-red-600">{quotesQuery.error.message}</p>
        )}

        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Ticker</TableHead>
              <TableHead>Price</TableHead>
              <TableHead>Source</TableHead>
              <TableHead>Fetched at</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {latest.length === 0 && (
              <TableRow>
                <TableCell colSpan={4} className="text-center text-muted-foreground">
                  No quotes collected yet.
                </TableCell>
              </TableRow>
            )}
            {latest.map((quote) => (
              <TableRow key={quote.ticker}>
                <TableCell>{quote.ticker}</TableCell>
                <TableCell>R$ {quote.price.toFixed(2)}</TableCell>
                <TableCell>{quote.source}</TableCell>
                <TableCell>{formatDateTime(quote.fetched_at)}</TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>

        <h3 className="text-sm font-medium">Fundamentals (Graham, Banks)</h3>
        {fundamentalsQuery.isError && (
          <p className="text-red-600">{fundamentalsQuery.error.message}</p>
        )}
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Ticker</TableHead>
              <TableHead>LPA</TableHead>
              <TableHead>VPA</TableHead>
              <TableHead>ROE</TableHead>
              <TableHead>Source</TableHead>
              <TableHead>Fetched at</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {latestFundamentals.length === 0 && (
              <TableRow>
                <TableCell colSpan={6} className="text-center text-muted-foreground">
                  No fundamentals collected yet.
                </TableCell>
              </TableRow>
            )}
            {latestFundamentals.map((item) => (
              <TableRow key={item.ticker}>
                <TableCell>{item.ticker}</TableCell>
                <TableCell>{item.lpa.toFixed(2)}</TableCell>
                <TableCell>{item.vpa.toFixed(2)}</TableCell>
                <TableCell>{item.roe.toFixed(2)}%</TableCell>
                <TableCell>{item.source}</TableCell>
                <TableCell>{formatDateTime(item.fetched_at)}</TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>

        <h3 className="text-sm font-medium">Average dividend, last 5 years (Bazin)</h3>
        {dividendsAvgQuery.isError && (
          <p className="text-red-600">{dividendsAvgQuery.error.message}</p>
        )}
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Ticker</TableHead>
              <TableHead>Avg dividend/share</TableHead>
              <TableHead>Source</TableHead>
              <TableHead>Fetched at</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {latestDividendsAvg.length === 0 && (
              <TableRow>
                <TableCell colSpan={4} className="text-center text-muted-foreground">
                  No dividend averages collected yet.
                </TableCell>
              </TableRow>
            )}
            {latestDividendsAvg.map((item) => (
              <TableRow key={item.ticker}>
                <TableCell>{item.ticker}</TableCell>
                <TableCell>R$ {item.avg_dividend_5y.toFixed(4)}</TableCell>
                <TableCell>{item.source}</TableCell>
                <TableCell>{formatDateTime(item.fetched_at)}</TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>

        <h3 className="text-sm font-medium">DCF fundamentals (CVM)</h3>
        {dcfFundamentalsQuery.isError && (
          <p className="text-red-600">{dcfFundamentalsQuery.error.message}</p>
        )}
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Ticker</TableHead>
              <TableHead>Ref. year</TableHead>
              <TableHead>EBIT</TableHead>
              <TableHead>D&amp;A</TableHead>
              <TableHead>Capex</TableHead>
              <TableHead>ΔNWC</TableHead>
              <TableHead>Total debt</TableHead>
              <TableHead>Cash</TableHead>
              <TableHead>Shares outstanding</TableHead>
              <TableHead>Fetched at</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {latestDcfFundamentals.length === 0 && (
              <TableRow>
                <TableCell colSpan={10} className="text-center text-muted-foreground">
                  No DCF fundamentals collected yet.
                </TableCell>
              </TableRow>
            )}
            {latestDcfFundamentals.map((item) => (
              <TableRow key={item.ticker}>
                <TableCell>{item.ticker}</TableCell>
                <TableCell>{item.reference_year}</TableCell>
                <TableCell>{item.ebit.toFixed(1)}</TableCell>
                <TableCell>
                  {item.depreciation_amortization?.toFixed(1) ?? "—"}
                </TableCell>
                <TableCell>{item.capex?.toFixed(1) ?? "—"}</TableCell>
                <TableCell>{item.nwc_change.toFixed(1)}</TableCell>
                <TableCell>{item.total_debt.toFixed(1)}</TableCell>
                <TableCell>{item.cash.toFixed(1)}</TableCell>
                <TableCell>{item.shares_outstanding.toFixed(1)}</TableCell>
                <TableCell>{formatDateTime(item.fetched_at)}</TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </CardContent>
    </Card>
  );
}

export default StockCollectorPanel;
