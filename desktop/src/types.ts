export type ValuationModel = {
  id: number;
  ticker: string;
  reference_year: number;
  current_price: number;
  model: string;
  fair_price: number | null;
  safety_margin: number | null;
  verdict: string | null;
  updated_at: string;
};

export type AppError = {
  code: string;
  message: string;
};
