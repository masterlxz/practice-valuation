import { useState, type FormEvent } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useMutation } from "@tanstack/react-query";
import type { AppError, ValuationModel } from "../types";
import ValuationResult from "../components/ValuationResult";
import Field from "../components/Field";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

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
    <Card>
      <CardHeader>
        <CardTitle>Fair Price (Bazin)</CardTitle>
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

          <Field label="Average dividend per share (R$, last 5 years)">
            <Input
              required
              type="number"
              step="0.01"
              value={averageDividend}
              onChange={(e) => setAverageDividend(e.currentTarget.value)}
            />
          </Field>

          <Field label="Desired yield (%)">
            <Input
              required
              type="number"
              step="0.1"
              value={desiredYield}
              onChange={(e) => setDesiredYield(e.currentTarget.value)}
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

export default BazinForm;
