import type { AppError, ValuationModel } from "../types";
import { Card, CardContent } from "@/components/ui/card";
import VerdictBadge from "./VerdictBadge";

type Props = {
  isError: boolean;
  error: AppError | null;
  isSuccess: boolean;
  valuation: ValuationModel | null;
};

function ValuationResult({ isError, error, isSuccess, valuation }: Props) {
  return (
    <>
      {isError && error && <p className="mt-6 text-red-600">{error.message}</p>}

      {isSuccess && valuation && (
        <Card className="mt-6">
          <CardContent className="flex flex-col gap-2">
            <p>
              Fair price: <strong>R$ {valuation.fair_price?.toFixed(2)}</strong>
            </p>
            <p>
              Safety margin:{" "}
              <strong>
                {((valuation.safety_margin ?? 0) * 100).toFixed(1)}%
              </strong>
            </p>
            <p className="flex items-center gap-2">
              Verdict: <VerdictBadge verdict={valuation.verdict} />
            </p>
          </CardContent>
        </Card>
      )}
    </>
  );
}

export default ValuationResult;
