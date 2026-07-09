import type { AppError, ValuationModel } from "../types";

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
        <div className="mt-6 rounded border p-4">
          <p>
            Fair price: <strong>R$ {valuation.fair_price?.toFixed(2)}</strong>
          </p>
          <p>
            Safety margin:{" "}
            <strong>{((valuation.safety_margin ?? 0) * 100).toFixed(1)}%</strong>
          </p>
          <p>
            Verdict: <strong>{valuation.verdict}</strong>
          </p>
        </div>
      )}
    </>
  );
}

export default ValuationResult;
