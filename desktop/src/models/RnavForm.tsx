import { useState, type FormEvent } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useMutation } from "@tanstack/react-query";
import type { AppError, ValuationModel } from "../types";
import ValuationResult from "../components/ValuationResult";

type CalculateRnavRequest = {
  ticker: string;
  reference_year: number;
  current_price: number;
  landbank: number;
  inventory_at_market_value: number;
  net_cash: number;
  shares_outstanding: number;
};

type RnavInputsModel = {
  id: number;
  valuation_id: number;
  landbank: number;
  inventory_at_market_value: number;
  net_cash: number;
  shares_outstanding: number;
};

type RnavValuationResponse = {
  valuation: ValuationModel;
  inputs: RnavInputsModel;
};

function RnavForm() {
  const [ticker, setTicker] = useState("");
  const [referenceYear, setReferenceYear] = useState(
    String(new Date().getFullYear()),
  );
  const [currentPrice, setCurrentPrice] = useState("");
  const [landbank, setLandbank] = useState("");
  const [inventoryAtMarketValue, setInventoryAtMarketValue] = useState("");
  const [netCash, setNetCash] = useState("");
  const [sharesOutstanding, setSharesOutstanding] = useState("");

  const mutation = useMutation<
    RnavValuationResponse,
    AppError,
    CalculateRnavRequest
  >({
    mutationFn: (request) => invoke("calculate_rnav", { request }),
  });

  function handleSubmit(event: FormEvent) {
    event.preventDefault();
    mutation.mutate({
      ticker: ticker.toUpperCase(),
      reference_year: Number(referenceYear),
      current_price: Number(currentPrice),
      landbank: Number(landbank),
      inventory_at_market_value: Number(inventoryAtMarketValue),
      net_cash: Number(netCash),
      shares_outstanding: Number(sharesOutstanding),
    });
  }

  return (
    <>
      <h1 className="mb-6 text-2xl font-semibold">Fair Price (RNAV)</h1>

      <form onSubmit={handleSubmit} className="flex flex-col gap-4">
        <label className="flex flex-col gap-1">
          Ticker
          <input
            required
            value={ticker}
            onChange={(e) => setTicker(e.currentTarget.value)}
            placeholder="CYRE3"
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
          Landbank at market value (R$ millions)
          <input
            required
            type="number"
            step="0.01"
            value={landbank}
            onChange={(e) => setLandbank(e.currentTarget.value)}
            className="rounded border px-3 py-2"
          />
        </label>

        <label className="flex flex-col gap-1">
          Inventory at market value (R$ millions)
          <input
            required
            type="number"
            step="0.01"
            value={inventoryAtMarketValue}
            onChange={(e) => setInventoryAtMarketValue(e.currentTarget.value)}
            className="rounded border px-3 py-2"
          />
        </label>

        <label className="flex flex-col gap-1">
          Net cash — cash minus debt, can be negative (R$ millions)
          <input
            required
            type="number"
            step="0.01"
            value={netCash}
            onChange={(e) => setNetCash(e.currentTarget.value)}
            className="rounded border px-3 py-2"
          />
        </label>

        <label className="flex flex-col gap-1">
          Shares outstanding (millions)
          <input
            required
            type="number"
            step="0.01"
            value={sharesOutstanding}
            onChange={(e) => setSharesOutstanding(e.currentTarget.value)}
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

export default RnavForm;
