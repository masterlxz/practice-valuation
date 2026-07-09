import { useState, type FormEvent } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useMutation } from "@tanstack/react-query";
import type { AppError, ValuationModel } from "../types";
import ValuationResult from "../components/ValuationResult";
import Field from "../components/Field";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

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
    <Card>
      <CardHeader>
        <CardTitle>Fair Price (Gordon / DDM)</CardTitle>
      </CardHeader>
      <CardContent>
        <form onSubmit={handleSubmit} className="flex flex-col gap-4">
          <Field label="Ticker">
            <Input
              required
              value={ticker}
              onChange={(e) => setTicker(e.currentTarget.value)}
              placeholder="ITSA4"
            />
          </Field>

          <Field label="Reference year">
            <Input
              required
              type="number"
              value={referenceYear}
              onChange={(e) => setReferenceYear(e.currentTarget.value)}
            />
          </Field>

          <Field label="Current price (R$)">
            <Input
              required
              type="number"
              step="0.01"
              value={currentPrice}
              onChange={(e) => setCurrentPrice(e.currentTarget.value)}
            />
          </Field>

          <Field label="Current dividend — D0 (R$/share)">
            <Input
              required
              type="number"
              step="0.01"
              value={currentDividend}
              onChange={(e) => setCurrentDividend(e.currentTarget.value)}
            />
          </Field>

          <Field label="Expected dividend growth — g (%)">
            <Input
              required
              type="number"
              step="0.1"
              value={expectedGrowth}
              onChange={(e) => setExpectedGrowth(e.currentTarget.value)}
            />
          </Field>

          <Field label="Required return — Ke (%)">
            <Input
              required
              type="number"
              step="0.1"
              value={ke}
              onChange={(e) => setKe(e.currentTarget.value)}
            />
          </Field>

          <Button type="submit" disabled={mutation.isPending}>
            {mutation.isPending ? "Calculating..." : "Calculate"}
          </Button>
        </form>

        <ValuationResult
          isError={mutation.isError}
          error={mutation.error ?? null}
          isSuccess={mutation.isSuccess}
          valuation={mutation.data?.valuation ?? null}
        />
      </CardContent>
    </Card>
  );
}

export default GordonForm;
