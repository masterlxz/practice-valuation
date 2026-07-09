import { useState, type FormEvent } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useMutation } from "@tanstack/react-query";
import type { AppError, ValuationModel } from "../types";
import ValuationResult from "../components/ValuationResult";

type CalculateGrahamRequest = {
  ticker: string;
  reference_year: number;
  current_price: number;
  eps: number;
  book_value_per_share: number;
};

type GrahamInputsModel = {
  id: number;
  valuation_id: number;
  eps: number;
  book_value_per_share: number;
};

type GrahamValuationResponse = {
  valuation: ValuationModel;
  inputs: GrahamInputsModel;
};

function GrahamForm() {
  const [ticker, setTicker] = useState("");
  const [referenceYear, setReferenceYear] = useState(
    String(new Date().getFullYear()),
  );
  const [currentPrice, setCurrentPrice] = useState("");
  const [eps, setEps] = useState("");
  const [bookValuePerShare, setBookValuePerShare] = useState("");

  const mutation = useMutation<
    GrahamValuationResponse,
    AppError,
    CalculateGrahamRequest
  >({
    mutationFn: (request) => invoke("calculate_graham", { request }),
  });

  function handleSubmit(event: FormEvent) {
    event.preventDefault();
    mutation.mutate({
      ticker: ticker.toUpperCase(),
      reference_year: Number(referenceYear),
      current_price: Number(currentPrice),
      eps: Number(eps),
      book_value_per_share: Number(bookValuePerShare),
    });
  }

  return (
    <>
      <h1 className="mb-6 text-2xl font-semibold">Fair Price (Graham)</h1>

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
          EPS — earnings per share (R$)
          <input
            required
            type="number"
            step="0.01"
            value={eps}
            onChange={(e) => setEps(e.currentTarget.value)}
            className="rounded border px-3 py-2"
          />
        </label>

        <label className="flex flex-col gap-1">
          Book value per share (R$)
          <input
            required
            type="number"
            step="0.01"
            value={bookValuePerShare}
            onChange={(e) => setBookValuePerShare(e.currentTarget.value)}
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

export default GrahamForm;
