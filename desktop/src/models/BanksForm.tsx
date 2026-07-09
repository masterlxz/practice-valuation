import { useState, type FormEvent } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useMutation } from "@tanstack/react-query";
import type { AppError, ValuationModel } from "../types";
import ValuationResult from "../components/ValuationResult";
import Field from "../components/Field";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

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
    <Card>
      <CardHeader>
        <CardTitle>Fair Price (Banks — P/B via ROE-Gordon)</CardTitle>
      </CardHeader>
      <CardContent>
        <form onSubmit={handleSubmit} className="flex flex-col gap-4">
          <Field label="Ticker">
            <Input
              required
              value={ticker}
              onChange={(e) => setTicker(e.currentTarget.value)}
              placeholder="ITUB4"
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

          <Field label="Book value per share (R$)">
            <Input
              required
              type="number"
              step="0.01"
              value={bookValuePerShare}
              onChange={(e) => setBookValuePerShare(e.currentTarget.value)}
            />
          </Field>

          <Field label="ROE (%)">
            <Input
              required
              type="number"
              step="0.1"
              value={roe}
              onChange={(e) => setRoe(e.currentTarget.value)}
            />
          </Field>

          <Field label="Payout (%)">
            <Input
              required
              type="number"
              step="0.1"
              value={payout}
              onChange={(e) => setPayout(e.currentTarget.value)}
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

export default BanksForm;
