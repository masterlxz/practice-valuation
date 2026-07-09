import { useState, type FormEvent } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import type { AppError } from "../types";
import { INDICATORS, INDICATOR_KEYS, type IndicatorKey } from "./indicators";

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
  GREEN: "text-green-700",
  NEUTRAL: "text-yellow-700",
  RED: "text-red-700",
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
    <>
      <h1 className="mb-6 text-2xl font-semibold">Crypto Score</h1>

      <form onSubmit={handleSubmit} className="flex flex-col gap-4">
        <label className="flex flex-col gap-1">
          Coin
          <input
            required
            value={coin}
            onChange={(e) => setCoin(e.currentTarget.value)}
            className="rounded border px-3 py-2"
          />
        </label>

        <label className="flex flex-col gap-1">
          Indicator
          <select
            value={indicator}
            onChange={(e) =>
              setIndicator(e.currentTarget.value as IndicatorKey)
            }
            className="rounded border px-3 py-2"
          >
            {INDICATOR_KEYS.map((key) => (
              <option key={key} value={key}>
                {INDICATORS[key]}
              </option>
            ))}
          </select>
        </label>

        <label className="flex flex-col gap-1">
          Reading date
          <input
            required
            type="date"
            value={readingDate}
            onChange={(e) => setReadingDate(e.currentTarget.value)}
            className="rounded border px-3 py-2"
          />
        </label>

        <label className="flex flex-col gap-1">
          Raw value (already normalized when the indicator is trend-based —
          e.g. % change for TVL/active addresses, ratio to 90d average for
          NVT)
          <input
            required
            type="number"
            step="any"
            value={rawValue}
            onChange={(e) => setRawValue(e.currentTarget.value)}
            className="rounded border px-3 py-2"
          />
        </label>

        <label className="flex flex-col gap-1">
          Source
          <input
            required
            value={source}
            onChange={(e) => setSource(e.currentTarget.value)}
            placeholder="ultrasound.money"
            className="rounded border px-3 py-2"
          />
        </label>

        <button
          type="submit"
          disabled={mutation.isPending}
          className="rounded bg-black px-4 py-2 text-white disabled:opacity-50"
        >
          {mutation.isPending ? "Recording..." : "Record reading"}
        </button>
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

        <table className="w-full border-collapse text-left text-sm">
          <thead>
            <tr className="border-b">
              <th className="py-1 pr-4">Indicator</th>
              <th className="py-1 pr-4">Value</th>
              <th className="py-1 pr-4">Signal</th>
              <th className="py-1 pr-4">Date</th>
            </tr>
          </thead>
          <tbody>
            {INDICATOR_KEYS.map((key) => {
              const reading = latest.get(key);
              return (
                <tr key={key} className="border-b">
                  <td className="py-1 pr-4">{INDICATORS[key]}</td>
                  <td className="py-1 pr-4">
                    {reading ? reading.raw_value : "—"}
                  </td>
                  <td
                    className={`py-1 pr-4 font-medium ${
                      reading ? SIGNAL_STYLE[reading.signal] : ""
                    }`}
                  >
                    {reading ? reading.signal : "not logged"}
                  </td>
                  <td className="py-1 pr-4">
                    {reading ? reading.reading_date : "—"}
                  </td>
                </tr>
              );
            })}
          </tbody>
        </table>
      </div>
    </>
  );
}

export default CryptoScorePanel;
