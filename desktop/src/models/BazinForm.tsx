import { useState, type FormEvent } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useMutation } from "@tanstack/react-query";
import type { AppError, ValuationModel } from "../types";
import ValuationResult from "../components/ValuationResult";

type CalculateBazinRequest = {
  ticker: string;
  reference_year: number;
  current_price: number;
  average_dividend: number;
  desired_yield: number;
};

type BazinInputsModel = {
  id: number;
  valuation_id: number;
  average_dividend: number;
  desired_yield: number;
};

type BazinValuationResponse = {
  valuation: ValuationModel;
  inputs: BazinInputsModel;
};

function BazinForm() {
  const [ticker, setTicker] = useState("");
  const [referenceYear, setReferenceYear] = useState(
    String(new Date().getFullYear()),
  );
  const [currentPrice, setCurrentPrice] = useState("");
  const [averageDividend, setAverageDividend] = useState("");
  const [desiredYield, setDesiredYield] = useState("6");

  const mutation = useMutation<
    BazinValuationResponse,
    AppError,
    CalculateBazinRequest
  >({
    mutationFn: (request) => invoke("calculate_bazin", { request }),
  });

  function handleSubmit(event: FormEvent) {
    event.preventDefault();
    mutation.mutate({
      ticker: ticker.toUpperCase(),
      reference_year: Number(referenceYear),
      current_price: Number(currentPrice),
      average_dividend: Number(averageDividend),
      desired_yield: Number(desiredYield) / 100,
    });
  }

  return (
    <>
      <h1 className="mb-6 text-2xl font-semibold">Fair Price (Bazin)</h1>

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
          Average dividend per share (R$, last 5 years)
          <input
            required
            type="number"
            step="0.01"
            value={averageDividend}
            onChange={(e) => setAverageDividend(e.currentTarget.value)}
            className="rounded border px-3 py-2"
          />
        </label>

        <label className="flex flex-col gap-1">
          Desired yield (%)
          <input
            required
            type="number"
            step="0.1"
            value={desiredYield}
            onChange={(e) => setDesiredYield(e.currentTarget.value)}
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

export default BazinForm;
