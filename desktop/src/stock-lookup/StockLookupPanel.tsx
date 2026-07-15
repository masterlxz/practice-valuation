import { useEffect, useRef, useState, type FormEvent } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import type { AppError, ValuationModel } from "../types";
import { latestForTicker } from "../collector/latestForTicker";
import type {
  StockDcfFundamentals,
  StockDividendsAvg,
  StockFundamentals,
  StockQuote,
  StockTechnicals,
} from "../collector/types";
import Field from "../components/Field";
import VerdictBadge from "../components/VerdictBadge";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Textarea } from "@/components/ui/textarea";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

type CollectorSummary = { success: boolean; output: string };

type StockNote = {
  id: number;
  ticker: string;
  note: string;
  updated_at: string;
};

type LookupData = {
  quote: StockQuote | null;
  fundamentals: StockFundamentals | null;
  dividendsAvg: StockDividendsAvg | null;
  dcfFundamentals: StockDcfFundamentals | null;
  technicals: StockTechnicals | null;
  note: StockNote | null;
};

const MODEL_LABELS: Record<string, string> = {
  bazin: "Bazin",
  graham: "Graham",
  gordon: "Gordon / DDM",
  dcf: "DCF / FCFF",
  banks: "Banks (P/B)",
  rim: "RIM (Bancos)",
  rnav: "RNAV",
  projected_ceiling: "Projected Ceiling",
};

function formatCurrency(value: number | null | undefined): string {
  return value == null ? "—" : `R$ ${value.toFixed(2)}`;
}

function formatPercent(value: number | null | undefined): string {
  return value == null ? "—" : `${value.toFixed(2)}%`;
}

function formatRatio(value: number | null): string {
  return value == null ? "—" : value.toFixed(2);
}

// Same "label / value" tile shape as CryptoScorePanel's IndicatorTile, reused
// here for quote/technicals/fundamentals/DCF stats instead of signals.
function StatTile({ label, value }: { label: string; value: string }) {
  return (
    <div className="rounded-lg border border-border bg-card p-4">
      <p className="text-sm text-muted-foreground">{label}</p>
      <p className="mt-1 text-2xl font-semibold">{value}</p>
    </div>
  );
}

function StockLookupPanel() {
  const [tickerInput, setTickerInput] = useState("");
  const [activeTicker, setActiveTicker] = useState<string | null>(null);
  const [noteDraft, setNoteDraft] = useState("");
  const autoFetchedTickerRef = useRef<string | null>(null);

  const queryClient = useQueryClient();

  const lookupQuery = useQuery<LookupData, AppError>({
    queryKey: ["stock-lookup", activeTicker],
    enabled: activeTicker !== null,
    queryFn: async () => {
      const ticker = activeTicker as string;
      const [quotes, fundamentals, dividendsAvg, dcfFundamentals, technicals, notes] =
        await Promise.all([
          invoke<StockQuote[]>("list_stock_quotes"),
          invoke<StockFundamentals[]>("list_stock_fundamentals"),
          invoke<StockDividendsAvg[]>("list_stock_dividends_avg"),
          invoke<StockDcfFundamentals[]>("list_stock_dcf_fundamentals"),
          invoke<StockTechnicals[]>("list_stock_technicals"),
          invoke<StockNote[]>("list_stock_notes"),
        ]);

      return {
        quote: latestForTicker(quotes, ticker),
        fundamentals: latestForTicker(fundamentals, ticker),
        dividendsAvg: latestForTicker(dividendsAvg, ticker),
        dcfFundamentals: latestForTicker(dcfFundamentals, ticker),
        technicals: latestForTicker(technicals, ticker),
        note: notes.find((n) => n.ticker === ticker) ?? null,
      };
    },
  });

  const valuationsQuery = useQuery<ValuationModel[], AppError>({
    queryKey: ["valuations"],
    queryFn: () => invoke("list_valuations"),
  });

  const collectorMutation = useMutation<CollectorSummary, AppError, string>({
    mutationFn: (ticker) => invoke<CollectorSummary>("run_stock_collector", { ticker }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["stock-lookup", activeTicker] });
    },
  });

  // Cache-aware fetch: a ticker with nothing in the DB yet gets exactly one
  // automatic collector run per search session (guarded by the ref, so a
  // genuinely-invalid ticker doesn't retrigger forever after invalidation).
  // Repeat searches for an already-collected ticker just read the DB —
  // "Refresh data" below is the only way to force a new fetch after that.
  useEffect(() => {
    if (
      activeTicker &&
      lookupQuery.isSuccess &&
      lookupQuery.data.quote === null &&
      autoFetchedTickerRef.current !== activeTicker
    ) {
      autoFetchedTickerRef.current = activeTicker;
      collectorMutation.mutate(activeTicker);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [activeTicker, lookupQuery.isSuccess, lookupQuery.data]);

  useEffect(() => {
    setNoteDraft(lookupQuery.data?.note?.note ?? "");
  }, [lookupQuery.data?.note]);

  const saveNoteMutation = useMutation<StockNote, AppError, void>({
    mutationFn: () =>
      invoke<StockNote>("save_stock_note", {
        request: { ticker: activeTicker, note: noteDraft },
      }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["stock-lookup", activeTicker] });
    },
  });

  function handleSearch(event: FormEvent) {
    event.preventDefault();
    const normalized = tickerInput.trim().toUpperCase();
    if (!normalized) return;
    setActiveTicker(normalized);
  }

  const data = lookupQuery.data;
  const price = data?.quote?.price ?? null;
  const lpa = data?.fundamentals?.lpa ?? null;
  const vpa = data?.fundamentals?.vpa ?? null;
  const pl = price != null && lpa ? price / lpa : null;
  const pvp = price != null && vpa ? price / vpa : null;

  const savedValuations = (valuationsQuery.data ?? []).filter(
    (v) => v.ticker === activeTicker,
  );
  const latestValuation = savedValuations[0] ?? null;

  return (
    <Card>
      <CardHeader>
        <CardTitle>Stock Lookup</CardTitle>
      </CardHeader>
      <CardContent className="flex flex-col gap-6">
        <form onSubmit={handleSearch} className="flex items-end gap-3">
          <Field label="Ticker" className="flex-1">
            <Input
              required
              value={tickerInput}
              onChange={(e) => setTickerInput(e.currentTarget.value)}
              placeholder="PETR4"
            />
          </Field>
          <Button type="submit">Search</Button>
          {activeTicker && (
            <Button
              type="button"
              variant="outline"
              onClick={() => collectorMutation.mutate(activeTicker)}
              disabled={collectorMutation.isPending}
            >
              {collectorMutation.isPending ? "Refreshing..." : "Refresh data"}
            </Button>
          )}
        </form>

        {lookupQuery.isError && (
          <p className="text-red-600">{lookupQuery.error.message}</p>
        )}
        {collectorMutation.isError && (
          <p className="text-red-600">{collectorMutation.error.message}</p>
        )}

        {activeTicker && lookupQuery.isLoading && (
          <p className="text-muted-foreground">Loading {activeTicker}...</p>
        )}

        {activeTicker && data?.quote === null && collectorMutation.isPending && (
          <p className="text-muted-foreground">
            Fetching {activeTicker} for the first time...
          </p>
        )}

        {activeTicker && data?.quote === null && !collectorMutation.isPending && (
          <p className="text-muted-foreground">No data found for {activeTicker}.</p>
        )}

        {activeTicker && data && data.quote !== null && (
          <>
            <div className="grid grid-cols-2 gap-3 sm:grid-cols-4">
              <StatTile label="Price" value={formatCurrency(price)} />
              <StatTile label="SMA 50" value={formatCurrency(data.technicals?.sma_50)} />
              <StatTile label="SMA 100" value={formatCurrency(data.technicals?.sma_100)} />
              <StatTile label="SMA 200" value={formatCurrency(data.technicals?.sma_200)} />
              <StatTile label="CAGR 5y" value={formatPercent(data.technicals?.cagr_5y)} />
              <StatTile label="CAGR 10y" value={formatPercent(data.technicals?.cagr_10y)} />
              <StatTile label="P/L" value={formatRatio(pl)} />
              <StatTile label="P/VP" value={formatRatio(pvp)} />
            </div>

            <div>
              <h3 className="mb-3 text-sm font-semibold text-muted-foreground">
                Fundamentals
              </h3>
              <div className="grid grid-cols-2 gap-3 sm:grid-cols-4">
                <StatTile label="LPA" value={formatRatio(data.fundamentals?.lpa ?? null)} />
                <StatTile label="VPA" value={formatRatio(data.fundamentals?.vpa ?? null)} />
                <StatTile label="ROE" value={formatPercent(data.fundamentals?.roe)} />
                <StatTile label="Payout" value={formatPercent(data.fundamentals?.payout)} />
                <StatTile
                  label="Avg dividend (5y)"
                  value={formatCurrency(data.dividendsAvg?.avg_dividend_5y)}
                />
              </div>
            </div>

            <div>
              <h3 className="mb-3 text-sm font-semibold text-muted-foreground">
                DCF fundamentals
              </h3>
              <div className="grid grid-cols-2 gap-3 sm:grid-cols-4">
                <StatTile label="EBIT" value={formatRatio(data.dcfFundamentals?.ebit ?? null)} />
                <StatTile label="Tax rate" value={formatPercent(data.dcfFundamentals?.tax_rate)} />
                <StatTile
                  label="Total debt"
                  value={formatRatio(data.dcfFundamentals?.total_debt ?? null)}
                />
                <StatTile label="Cash" value={formatRatio(data.dcfFundamentals?.cash ?? null)} />
              </div>
            </div>

            <div>
              <h3 className="mb-3 text-sm font-semibold text-muted-foreground">
                Saved valuation
              </h3>
              {latestValuation ? (
                <div className="rounded-lg border border-border bg-card p-4">
                  <p className="text-sm text-muted-foreground">
                    {MODEL_LABELS[latestValuation.model] ?? latestValuation.model}
                  </p>
                  <p className="mt-1 text-2xl font-semibold">
                    {formatCurrency(latestValuation.fair_price)}
                  </p>
                  <div className="mt-2">
                    <VerdictBadge verdict={latestValuation.verdict} />
                  </div>
                </div>
              ) : (
                <p className="text-muted-foreground">
                  No saved valuation for {activeTicker} yet.
                </p>
              )}
            </div>

            <div>
              <h3 className="mb-3 text-sm font-semibold text-muted-foreground">Notes</h3>
              <Textarea
                value={noteDraft}
                onChange={(e) => setNoteDraft(e.currentTarget.value)}
                rows={4}
              />
              <Button
                type="button"
                className="mt-2"
                onClick={() => saveNoteMutation.mutate()}
                disabled={saveNoteMutation.isPending}
              >
                {saveNoteMutation.isPending ? "Saving..." : "Save notes"}
              </Button>
              {saveNoteMutation.isError && (
                <p className="mt-2 text-red-600">{saveNoteMutation.error.message}</p>
              )}
            </div>
          </>
        )}
      </CardContent>
    </Card>
  );
}

export default StockLookupPanel;
