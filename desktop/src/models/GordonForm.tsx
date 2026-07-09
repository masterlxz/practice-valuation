import { useState, type FormEvent } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useMutation } from "@tanstack/react-query";
import type { AppError, ValuationModel } from "../types";
import ValuationResult from "../components/ValuationResult";

type CalculateGordonRequest = {
  ticker: string;
  reference_year: number;
  current_price: number;
  current_dividend: number;
  expected_growth: number;
  ke: number;
};

type GordonInputsModel = {
  id: number;
  valuation_id: number;
  current_dividend: number;
  expected_growth: number;
  ke: number;
};

type GordonValuationResponse = {
  valuation: ValuationModel;
  inputs: GordonInputsModel;
};

function GordonForm() {
  const [ticker, setTicker] = useState("");
  const [referenceYear, setReferenceYear] = useState(
    String(new Date().getFullYear()),
  );
  const [currentPrice, setCurrentPrice] = useState("");
  const [currentDividend, setCurrentDividend] = useState("");
  const [expectedGrowth, setExpectedGrowth] = useState("");
  const [ke, setKe] = useState("");

  const mutation = useMutation<
    GordonValuationResponse,
    AppError,
    CalculateGordonRequest
  >({
    mutationFn: (request) => invoke("calculate_gordon", { request }),
  });

  function handleSubmit(event: FormEvent) {
    event.preventDefault();
    mutation.mutate({
      ticker: ticker.toUpperCase(),
      reference_year: Number(referenceYear),
      current_price: Number(currentPrice),
      current_dividend: Number(currentDividend),
      expected_growth: Number(expectedGrowth) / 100,
      ke: Number(ke) / 100,
    });
  }

  return (
    <>
      <h1 className="mb-6 text-2xl font-semibold">Fair Price (Gordon / DDM)</h1>

      <form onSubmit={handleSubmit} className="flex flex-col gap-4">
        <label className="flex flex-col gap-1">
          Ticker
          <input
            required
            value={ticker}
            onChange={(e) => setTicker(e.currentTarget.value)}
            placeholder="ITSA4"
            className="rounded border px-3 py-2"
          />
        </label>

        <label className="flex flex-col gap-1">
          Reference year
          <input
            required
            type="number"
            value={referenceYear}
            onChange={(e) => setReferenceYear(e.currentTarget.value)}
            className="rounded border px-3 py-2"
          />
        </label>

        <label className="flex flex-col gap-1">
          Current price (R$)
          <input
            required
            type="number"
            step="0.01"
            value={currentPrice}
            onChange={(e) => setCurrentPrice(e.currentTarget.value)}
            className="rounded border px-3 py-2"
          />
        </label>

        <label className="flex flex-col gap-1">
          Current dividend — D0 (R$/share)
          <input
            required
            type="number"
            step="0.01"
            value={currentDividend}
            onChange={(e) => setCurrentDividend(e.currentTarget.value)}
            className="rounded border px-3 py-2"
          />
        </label>

        <label className="flex flex-col gap-1">
          Expected dividend growth — g (%)
          <input
            required
            type="number"
            step="0.1"
            value={expectedGrowth}
            onChange={(e) => setExpectedGrowth(e.currentTarget.value)}
            className="rounded border px-3 py-2"
          />
        </label>

        <label className="flex flex-col gap-1">
          Required return — Ke (%)
          <input
            required
            type="number"
            step="0.1"
            value={ke}
            onChange={(e) => setKe(e.currentTarget.value)}
            className="rounded border px-3 py-2"
          />
        </label>

        <button
          type="submit"
          disabled={mutation.isPending}
          className="rounded bg-black px-4 py-2 text-white disabled:opacity-50"
        >
          {mutation.isPending ? "Calculating..." : "Calculate"}
        </button>
      </form>

      <ValuationResult
        isError={mutation.isError}
        error={mutation.error ?? null}
        isSuccess={mutation.isSuccess}
        valuation={mutation.data?.valuation ?? null}
      />
    </>
  );
}

export default GordonForm;
