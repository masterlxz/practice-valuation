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
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
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

function CryptoScorePanel() {
  const [coin, setCoin] = useState("ETH");
  const [indicator, setIndicator] = useState<IndicatorKey>(INDICATOR_KEYS[0]);
  const [readingDate, setReadingDate] = useState(today());
  const [rawValue, setRawValue] = useState("");
  const [source, setSource] = useState("");

  const queryClient = useQueryClient();

  const readingsQuery = useQuery<CryptoIndicatorReading[], AppError>({
    queryKey: ["crypto-indicators", coin],
    queryFn: () => invoke("list_crypto_indicators", { coin }),
  });

  const mutation = useMutation<
    CryptoIndicatorReading,
    AppError,
    RecordCryptoIndicatorRequest
  >({
    mutationFn: (request) => invoke("record_crypto_indicator", { request }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["crypto-indicators", coin] });
      setRawValue("");
      setSource("");
    },
  });

  function handleSubmit(event: FormEvent) {
    event.preventDefault();
    mutation.mutate({
      coin: coin.toUpperCase(),
      indicator,
      reading_date: readingDate,
      raw_value: Number(rawValue),
      source,
    });
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
        <form onSubmit={handleSubmit} className="flex flex-col gap-4">
          <Field label="Coin">
            <Input
              required
              value={coin}
              onChange={(e) => setCoin(e.currentTarget.value)}
            />
          </Field>

          <Field label="Indicator">
            <Select
              value={indicator}
              onValueChange={(value) => setIndicator(value as IndicatorKey)}
            >
              <SelectTrigger className="w-full">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                {INDICATOR_KEYS.map((key) => (
                  <SelectItem key={key} value={key}>
                    {INDICATORS[key]}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </Field>

          <Field label="Reading date">
            <Input
              required
              type="date"
              value={readingDate}
              onChange={(e) => setReadingDate(e.currentTarget.value)}
            />
          </Field>

          <Field label="Raw value (already normalized when the indicator is trend-based — e.g. % change for TVL/active addresses, ratio to 90d average for NVT)">
            <Input
              required
              type="number"
              step="any"
              value={rawValue}
              onChange={(e) => setRawValue(e.currentTarget.value)}
            />
          </Field>

          <Field label="Source">
            <Input
              required
              value={source}
              onChange={(e) => setSource(e.currentTarget.value)}
              placeholder="ultrasound.money"
            />
          </Field>

          <Button type="submit" disabled={mutation.isPending}>
            {mutation.isPending ? "Recording..." : "Record reading"}
          </Button>
        </form>

        {mutation.isError && (
          <p className="mt-6 text-red-600">{mutation.error.message}</p>
        )}

        <div className="mt-8">
          <p className="mb-2 font-medium">
            Green: {greenCount}/9 ({latest.size} of 9 indicators logged)
          </p>

          {readingsQuery.isError && (
            <p className="text-red-600">{readingsQuery.error.message}</p>
          )}

          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Indicator</TableHead>
                <TableHead>Value</TableHead>
                <TableHead>Signal</TableHead>
                <TableHead>Date</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {INDICATOR_KEYS.map((key) => {
                const reading = latest.get(key);
                return (
                  <TableRow key={key}>
                    <TableCell className="whitespace-normal">
                      {INDICATORS[key]}
                    </TableCell>
                    <TableCell>{reading ? reading.raw_value : "—"}</TableCell>
                    <TableCell>
                      {reading ? (
                        <Badge className={SIGNAL_STYLE[reading.signal]}>
                          {reading.signal}
                        </Badge>
                      ) : (
                        <span className="text-muted-foreground">
                          not logged
                        </span>
                      )}
                    </TableCell>
                    <TableCell>
                      {reading ? reading.reading_date : "—"}
                    </TableCell>
                  </TableRow>
                );
              })}
            </TableBody>
          </Table>
        </div>
      </CardContent>
    </Card>
  );
}

export default CryptoScorePanel;
