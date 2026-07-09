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

// `list_stock_quotes` orders by fetched_at desc, so the first row seen per
// ticker while iterating is the latest one — same pattern used for crypto
// readings and saved valuations.
function latestPerTicker(quotes: StockQuote[]): Map<string, StockQuote> {
  const latest = new Map<string, StockQuote>();
  for (const quote of quotes) {
    if (!latest.has(quote.ticker)) {
      latest.set(quote.ticker, quote);
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

  const runMutation = useMutation<CollectorSummary, AppError, void>({
    mutationFn: () => invoke("run_stock_collector"),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["stock-quotes"] });
    },
  });

  const latest = [...latestPerTicker(quotesQuery.data ?? []).values()];

  return (
    <Card>
      <CardHeader>
        <CardTitle>Data Collector — Stock Quotes</CardTitle>
      </CardHeader>
      <CardContent className="flex flex-col gap-4">
        <p className="text-sm text-muted-foreground">
          Runs the Python collector (brapi) for the tickers configured in{" "}
          <code>data-collector/config.yaml</code> and saves each quote as a
          new row — nothing is overwritten.
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
      </CardContent>
    </Card>
  );
}

export default StockCollectorPanel;
