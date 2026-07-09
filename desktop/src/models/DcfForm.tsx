import { useState, type FormEvent } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useMutation } from "@tanstack/react-query";
import type { AppError, ValuationModel } from "../types";
import ValuationResult from "../components/ValuationResult";

type CalculateDcfRequest = {
  ticker: string;
  reference_year: number;
  current_price: number;
  ebit: number;
  tax_rate: number;
  depreciation_amortization: number;
  capex: number;
  nwc_change: number;
  total_debt: number;
  cash: number;
  shares_outstanding: number;
  beta: number;
  risk_free_rate: number;
  market_risk_premium: number;
  kd: number;
  perpetuity_growth: number;
};

type DcfInputsModel = {
  id: number;
  valuation_id: number;
  ebit: number;
  tax_rate: number;
  depreciation_amortization: number;
  capex: number;
  nwc_change: number;
  total_debt: number;
  cash: number;
  shares_outstanding: number;
  beta: number;
  risk_free_rate: number;
  market_risk_premium: number;
  kd: number;
  perpetuity_growth: number;
};

type DcfValuationResponse = {
  valuation: ValuationModel;
  inputs: DcfInputsModel;
};

const fieldClass = "rounded border px-3 py-2";
const labelClass = "flex flex-col gap-1";

function DcfForm() {
  const [ticker, setTicker] = useState("");
  const [referenceYear, setReferenceYear] = useState(
    String(new Date().getFullYear()),
  );
  const [currentPrice, setCurrentPrice] = useState("");

  const [ebit, setEbit] = useState("");
  const [taxRate, setTaxRate] = useState("");
  const [depreciationAmortization, setDepreciationAmortization] = useState("");
  const [capex, setCapex] = useState("");
  const [nwcChange, setNwcChange] = useState("");

  const [totalDebt, setTotalDebt] = useState("");
  const [cash, setCash] = useState("");
  const [sharesOutstanding, setSharesOutstanding] = useState("");

  const [beta, setBeta] = useState("");
  const [riskFreeRate, setRiskFreeRate] = useState("");
  const [marketRiskPremium, setMarketRiskPremium] = useState("");
  const [kd, setKd] = useState("");
  const [perpetuityGrowth, setPerpetuityGrowth] = useState("");

  const mutation = useMutation<DcfValuationResponse, AppError, CalculateDcfRequest>(
    {
      mutationFn: (request) => invoke("calculate_dcf", { request }),
    },
  );

  function handleSubmit(event: FormEvent) {
    event.preventDefault();
    mutation.mutate({
      ticker: ticker.toUpperCase(),
      reference_year: Number(referenceYear),
      current_price: Number(currentPrice),
      ebit: Number(ebit),
      tax_rate: Number(taxRate) / 100,
      depreciation_amortization: Number(depreciationAmortization),
      capex: Number(capex),
      nwc_change: Number(nwcChange),
      total_debt: Number(totalDebt),
      cash: Number(cash),
      shares_outstanding: Number(sharesOutstanding),
      beta: Number(beta),
      risk_free_rate: Number(riskFreeRate) / 100,
      market_risk_premium: Number(marketRiskPremium) / 100,
      kd: Number(kd) / 100,
      perpetuity_growth: Number(perpetuityGrowth) / 100,
    });
  }

  return (
    <>
      <h1 className="mb-6 text-2xl font-semibold">Fair Price (DCF / FCFF)</h1>

      <form onSubmit={handleSubmit} className="flex flex-col gap-4">
        <label className={labelClass}>
          Ticker
          <input
            required
            value={ticker}
            onChange={(e) => setTicker(e.currentTarget.value)}
            placeholder="ITSA4"
            className={fieldClass}
          />
        </label>

        <label className={labelClass}>
          Reference year
          <input
            required
            type="number"
            value={referenceYear}
            onChange={(e) => setReferenceYear(e.currentTarget.value)}
            className={fieldClass}
          />
        </label>

        <label className={labelClass}>
          Current price (R$)
          <input
            required
            type="number"
            step="0.01"
            value={currentPrice}
            onChange={(e) => setCurrentPrice(e.currentTarget.value)}
            className={fieldClass}
          />
        </label>

        <h2 className="mt-2 text-sm font-semibold text-gray-500">Operational</h2>

        <label className={labelClass}>
          EBIT (R$ millions)
          <input
            required
            type="number"
            step="0.01"
            value={ebit}
            onChange={(e) => setEbit(e.currentTarget.value)}
            className={fieldClass}
          />
        </label>

        <label className={labelClass}>
          Effective tax rate (%)
          <input
            required
            type="number"
            step="0.1"
            value={taxRate}
            onChange={(e) => setTaxRate(e.currentTarget.value)}
            className={fieldClass}
          />
        </label>

        <label className={labelClass}>
          D&amp;A — depreciation/amortization (R$ millions)
          <input
            required
            type="number"
            step="0.01"
            value={depreciationAmortization}
            onChange={(e) => setDepreciationAmortization(e.currentTarget.value)}
            className={fieldClass}
          />
        </label>

        <label className={labelClass}>
          Capex (R$ millions)
          <input
            required
            type="number"
            step="0.01"
            value={capex}
            onChange={(e) => setCapex(e.currentTarget.value)}
            className={fieldClass}
          />
        </label>

        <label className={labelClass}>
          ΔNWC — change in net working capital (R$ millions)
          <input
            required
            type="number"
            step="0.01"
            value={nwcChange}
            onChange={(e) => setNwcChange(e.currentTarget.value)}
            className={fieldClass}
          />
        </label>

        <h2 className="mt-2 text-sm font-semibold text-gray-500">
          Capital structure
        </h2>

        <label className={labelClass}>
          Total debt (R$ millions)
          <input
            required
            type="number"
            step="0.01"
            value={totalDebt}
            onChange={(e) => setTotalDebt(e.currentTarget.value)}
            className={fieldClass}
          />
        </label>

        <label className={labelClass}>
          Cash (R$ millions)
          <input
            required
            type="number"
            step="0.01"
            value={cash}
            onChange={(e) => setCash(e.currentTarget.value)}
            className={fieldClass}
          />
        </label>

        <label className={labelClass}>
          Shares outstanding (millions)
          <input
            required
            type="number"
            step="0.01"
            value={sharesOutstanding}
            onChange={(e) => setSharesOutstanding(e.currentTarget.value)}
            className={fieldClass}
          />
        </label>

        <h2 className="mt-2 text-sm font-semibold text-gray-500">
          Cost of capital
        </h2>

        <label className={labelClass}>
          Beta
          <input
            required
            type="number"
            step="0.01"
            value={beta}
            onChange={(e) => setBeta(e.currentTarget.value)}
            className={fieldClass}
          />
        </label>

        <label className={labelClass}>
          Risk-free rate — Rf (%)
          <input
            required
            type="number"
            step="0.1"
            value={riskFreeRate}
            onChange={(e) => setRiskFreeRate(e.currentTarget.value)}
            className={fieldClass}
          />
        </label>

        <label className={labelClass}>
          Market risk premium (%)
          <input
            required
            type="number"
            step="0.1"
            value={marketRiskPremium}
            onChange={(e) => setMarketRiskPremium(e.currentTarget.value)}
            className={fieldClass}
          />
        </label>

        <label className={labelClass}>
          Cost of debt — Kd (%)
          <input
            required
            type="number"
            step="0.1"
            value={kd}
            onChange={(e) => setKd(e.currentTarget.value)}
            className={fieldClass}
          />
        </label>

        <label className={labelClass}>
          Perpetuity growth — g (%)
          <input
            required
            type="number"
            step="0.1"
            value={perpetuityGrowth}
            onChange={(e) => setPerpetuityGrowth(e.currentTarget.value)}
            className={fieldClass}
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

export default DcfForm;
