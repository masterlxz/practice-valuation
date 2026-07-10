import { useState, type FormEvent } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import type { AppError } from "../types";
import { INDICATORS, INDICATOR_KEYS, type IndicatorKey } from "./indicators";
import Field from "../components/Field";
import { Input } from "@/components/ui/input";
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

type RecordCryptoIndicatorRequest = {
  coin: string;
  indicator: string;
  reading_date: string;
  raw_value: number;
  source: string;
};

type CollectorSummary = {
  success: boolean;
  output: string;
};

type CryptoIndicatorReading = {
  id: number;
  coin: string;
  indicator: string;
  reading_date: string;
  raw_value: number;
  signal: "GREEN" | "NEUTRAL" | "RED";
  source: string;
  created_at: string;
};

const SIGNAL_STYLE: Record<CryptoIndicatorReading["signal"], string> = {
  GREEN: "bg-green-100 text-green-800 dark:bg-green-950 dark:text-green-300",
  NEUTRAL:
    "bg-yellow-100 text-yellow-800 dark:bg-yellow-950 dark:text-yellow-300",
  RED: "bg-red-100 text-red-800 dark:bg-red-950 dark:text-red-300",
};

type DraftRow = { rawValue: string; source: string };
type Drafts = Record<IndicatorKey, DraftRow>;

function emptyDrafts(): Drafts {
  return Object.fromEntries(
    INDICATOR_KEYS.map((key) => [key, { rawValue: "", source: "" }]),
  ) as Drafts;
}

function today(): string {
  return new Date().toISOString().slice(0, 10);
}

// Readings come back as a flat time series (one row per logged date); the
// score only cares about the most recent reading per indicator.
function latestPerIndicator(
  readings: CryptoIndicatorReading[],
): Map<string, CryptoIndicatorReading> {
  const latest = new Map<string, CryptoIndicatorReading>();
  for (const reading of readings) {
    const current = latest.get(reading.indicator);
    if (!current || reading.reading_date > current.reading_date) {
      latest.set(reading.indicator, reading);
    }
  }
  return latest;
}

// KPI-row stat tile (one per indicator) — label, value, status. A 9-row
// table read as a form; a grid of tiles reads as a dashboard.
function IndicatorTile({
  indicatorKey,
  reading,
}: {
  indicatorKey: IndicatorKey;
  reading?: CryptoIndicatorReading;
}) {
  return (
    <div className="rounded-lg border border-border bg-card p-4">
      <p className="text-sm text-muted-foreground">
        {INDICATORS[indicatorKey]}
      </p>
      <p className="mt-1 text-2xl font-semibold">
        {reading ? reading.raw_value.toFixed(2) : "—"}
      </p>
      <div className="mt-2 flex items-center justify-between gap-2">
        {reading ? (
          <Badge className={SIGNAL_STYLE[reading.signal]}>
            {reading.signal}
          </Badge>
        ) : (
          <span className="text-sm text-muted-foreground">not logged</span>
        )}
        {reading && (
          <span className="text-xs text-muted-foreground">
            {reading.reading_date}
          </span>
        )}
      </div>
    </div>
  );
}

function CryptoScorePanel() {
  const [coin, setCoin] = useState("ETH");
  const [readingDate, setReadingDate] = useState(today());
  const [drafts, setDrafts] = useState<Drafts>(emptyDrafts());

  const queryClient = useQueryClient();

  const readingsQuery = useQuery<CryptoIndicatorReading[], AppError>({
    queryKey: ["crypto-indicators", coin],
    queryFn: () => invoke("list_crypto_indicators", { coin }),
  });

  // Only `tvl_trend` (DefiLlama) and `net_issuance` (ultrasound.money) are
  // automated so far, both signup-free — the other 7 indicators still go
  // through the manual form below (paid sources or sources that need
  // investigation, see PROJECT_STATE.md).
  const runCryptoCollectorMutation = useMutation<CollectorSummary, AppError, void>({
    mutationFn: () => invoke("run_crypto_collector"),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["crypto-indicators", coin] });
    },
  });

  // One backend call per filled-in indicator — there's no dedicated "batch
  // insert" command, `record_crypto_indicator` already does exactly what
  // each row needs (look up the threshold, classify, persist), so a single
  // "Update all" click just fires it once per row that has a value.
  const updateAllMutation = useMutation<void, AppError, void>({
    mutationFn: async () => {
      const entries = INDICATOR_KEYS.filter(
        (key) => drafts[key].rawValue.trim() !== "",
      );
      await Promise.all(
        entries.map((indicator) => {
          const request: RecordCryptoIndicatorRequest = {
            coin: coin.toUpperCase(),
            indicator,
            reading_date: readingDate,
            raw_value: Number(drafts[indicator].rawValue),
            source: drafts[indicator].source,
          };
          return invoke("record_crypto_indicator", { request });
        }),
      );
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["crypto-indicators", coin] });
      setDrafts(emptyDrafts());
    },
  });

  function handleSubmit(event: FormEvent) {
    event.preventDefault();
    updateAllMutation.mutate();
  }

  function updateDraft(key: IndicatorKey, field: keyof DraftRow, value: string) {
    setDrafts((current) => ({
      ...current,
      [key]: { ...current[key], [field]: value },
    }));
  }

  const latest = latestPerIndicator(readingsQuery.data ?? []);
  const greenCount = [...latest.values()].filter(
    (reading) => reading.signal === "GREEN",
  ).length;

  return (
    <Card>
      <CardHeader>
        <CardTitle>Crypto Score</CardTitle>
      </CardHeader>
      <CardContent>
        <p className="mb-3 text-lg font-medium">
          Green: {greenCount}/9 ({latest.size} of 9 indicators logged)
        </p>

        <div className="mb-4 flex flex-col gap-2">
          <Button
            type="button"
            variant="outline"
            className="w-fit"
            onClick={() => runCryptoCollectorMutation.mutate()}
            disabled={runCryptoCollectorMutation.isPending}
          >
            {runCryptoCollectorMutation.isPending
              ? "Running..."
              : "Run crypto collector (TVL Trend, Net Issuance)"}
          </Button>
          {runCryptoCollectorMutation.isSuccess &&
            !runCryptoCollectorMutation.data.success && (
              <p className="whitespace-pre-wrap text-red-600">
                {runCryptoCollectorMutation.data.output}
              </p>
            )}
          {runCryptoCollectorMutation.isError && (
            <p className="text-red-600">
              {runCryptoCollectorMutation.error.message}
            </p>
          )}
        </div>

        {readingsQuery.isError && (
          <p className="mb-3 text-red-600">{readingsQuery.error.message}</p>
        )}

        <div className="grid grid-cols-1 gap-3 sm:grid-cols-3">
          {INDICATOR_KEYS.map((key) => (
            <IndicatorTile key={key} indicatorKey={key} reading={latest.get(key)} />
          ))}
        </div>

        <h3 className="mt-8 mb-3 text-sm font-semibold text-muted-foreground">
          Update readings
        </h3>

        <form onSubmit={handleSubmit} className="flex flex-col gap-4">
          <div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
            <Field label="Coin">
              <Input
                required
                value={coin}
                onChange={(e) => setCoin(e.currentTarget.value)}
              />
            </Field>

            <Field label="Reading date">
              <Input
                required
                type="date"
                value={readingDate}
                onChange={(e) => setReadingDate(e.currentTarget.value)}
              />
            </Field>
          </div>

          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Indicator</TableHead>
                <TableHead>Value</TableHead>
                <TableHead>Source</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {INDICATOR_KEYS.map((key) => (
                <TableRow key={key}>
                  <TableCell className="whitespace-normal">
                    {INDICATORS[key]}
                  </TableCell>
                  <TableCell>
                    <Input
                      type="number"
                      step="any"
                      value={drafts[key].rawValue}
                      onChange={(e) =>
                        updateDraft(key, "rawValue", e.currentTarget.value)
                      }
                    />
                  </TableCell>
                  <TableCell>
                    <Input
                      value={drafts[key].source}
                      onChange={(e) =>
                        updateDraft(key, "source", e.currentTarget.value)
                      }
                      placeholder="ultrasound.money"
                    />
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>

          {updateAllMutation.isError && (
            <p className="text-red-600">{updateAllMutation.error.message}</p>
          )}

          <Button type="submit" disabled={updateAllMutation.isPending}>
            {updateAllMutation.isPending ? "Updating..." : "Update all"}
          </Button>
        </form>
      </CardContent>
    </Card>
  );
}

export default CryptoScorePanel;
