import { useState, type FormEvent } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useMutation } from "@tanstack/react-query";
import { RefreshCw } from "lucide-react";
import type { AppError, ValuationModel } from "../types";
import { useTickerCollector } from "../collector/useTickerCollector";
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

  const tickerCollector = useTickerCollector();
  const [tickerError, setTickerError] = useState<string | null>(null);

  const mutation = useMutation<
    BanksValuationResponse,
    AppError,
    CalculateBanksRequest
  >({
    mutationFn: (request) => invoke("calculate_banks", { request }),
  });

  async function handleFetch() {
    if (!ticker.trim()) {
      setTickerError("Ticker is required to fetch data.");
      return;
    }
    setTickerError(null);
    const data = await tickerCollector.mutateAsync(ticker).catch(() => null);
    if (!data) return;
    let filled = 0;
    if (data.quote) {
      setCurrentPrice(String(data.quote.price));
      filled++;
    }
    if (data.fundamentals) {
      setBookValuePerShare(String(data.fundamentals.vpa));
      setRoe(String(data.fundamentals.roe));
      filled++;
    }
    if (filled === 0) {
      setTickerError(`No data found for ${ticker.toUpperCase()}.`);
    }
  }

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
        <form
          onSubmit={handleSubmit}
          className="grid grid-cols-1 gap-4 sm:grid-cols-2"
        >
          <Field label="Ticker">
            <div className="flex gap-2">
              <Input
                required
                value={ticker}
                onChange={(e) => setTicker(e.currentTarget.value)}
                placeholder="ITUB4"
                className="flex-1"
              />
              <Button
                type="button"
                variant="outline"
                size="icon"
                onClick={handleFetch}
                disabled={tickerCollector.isPending}
                aria-label="Fetch data for this ticker"
                title="Fetch data for this ticker"
              >
                <RefreshCw
                  className={tickerCollector.isPending ? "animate-spin" : ""}
                />
              </Button>
            </div>
            {tickerError && <p className="text-red-600">{tickerError}</p>}
            {tickerCollector.isError && (
              <p className="text-red-600">{tickerCollector.error.message}</p>
            )}
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

          <Button
            type="submit"
            disabled={mutation.isPending}
            className="sm:col-span-2"
          >
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
