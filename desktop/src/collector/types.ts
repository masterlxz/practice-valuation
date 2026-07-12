export type StockQuote = {
  id: number;
  ticker: string;
  price: number;
  source: string;
  fetched_at: string;
};

export type StockFundamentals = {
  id: number;
  ticker: string;
  lpa: number;
  vpa: number;
  roe: number;
  source: string;
  fetched_at: string;
};

export type StockDividendsAvg = {
  id: number;
  ticker: string;
  avg_dividend_5y: number;
  source: string;
  fetched_at: string;
};

export type StockDcfFundamentals = {
  id: number;
  ticker: string;
  reference_year: number;
  ebit: number;
  tax_rate: number | null;
  depreciation_amortization: number | null;
  capex: number | null;
  nwc_change: number;
  total_debt: number;
  cash: number;
  shares_outstanding: number;
  source: string;
  fetched_at: string;
};
