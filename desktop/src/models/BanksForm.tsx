import { useState, type FormEvent } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useMutation } from "@tanstack/react-query";
import type { AppError, ValuationModel } from "../types";
import ValuationResult from "../components/ValuationResult";

type CalculateBanksRequest = {
  ticker: string;
  reference_year: number;
  current_price: number;
  book_value_per_share: number;
  roe: number;
  payout: number;
  ke: number;
};

type BanksInputsModel = {
  id: number;
  valuation_id: number;
  book_value_per_share: number;
  roe: number;
  payout: number;
  ke: number;
};

type BanksValuationResponse = {
  valuation: ValuationModel;
  inputs: BanksInputsModel;
};

function BanksForm() {
  const [ticker, setTicker] = useState("");
  const [referenceYear, setReferenceYear] = useState(
    String(new Date().getFullYear()),
  );
  const [currentPrice, setCurrentPrice] = useState("");
  const [bookValuePerShare, setBookValuePerShare] = useState("");
  const [roe, setRoe] = useState("");
  const [payout, setPayout] = useState("");
  const [ke, setKe] = useState("");

  const mutation = useMutation<
    BanksValuationResponse,
    AppError,
    CalculateBanksRequest
  >({
    mutationFn: (request) => invoke("calculate_banks", { request }),
  });

  function handleSubmit(event: FormEvent) {
    event.preventDefault();
    mutation.mutate({
      ticker: ticker.toUpperCase(),
      reference_year: Number(referenceYear),
      current_price: Number(currentPrice),
      book_value_per_share: Number(bookValuePerShare),
      roe: Number(roe) / 100,
      payout: Number(payout) / 100,
      ke: Number(ke) / 100,
    });
  }

  return (
    <>
      <h1 className="mb-6 text-2xl font-semibold">
        Fair Price (Banks — P/B via ROE-Gordon)
      </h1>

      <form onSubmit={handleSubmit} className="flex flex-col gap-4">
        <label className="flex flex-col gap-1">
          Ticker
          <input
            required
            value={ticker}
            onChange={(e) => setTicker(e.currentTarget.value)}
            placeholder="ITUB4"
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

        <label className="flex flex-col gap-1">
          ROE (%)
          <input
            required
            type="number"
            step="0.1"
            value={roe}
            onChange={(e) => setRoe(e.currentTarget.value)}
            className="rounded border px-3 py-2"
          />
        </label>

        <label className="flex flex-col gap-1">
          Payout (%)
          <input
            required
            type="number"
            step="0.1"
            value={payout}
            onChange={(e) => setPayout(e.currentTarget.value)}
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

export default BanksForm;
