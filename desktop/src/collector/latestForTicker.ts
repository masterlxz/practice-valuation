// `list_stock_*` commands order by fetched_at desc, so the first row
// matching the ticker while iterating is the latest one.
export function latestForTicker<T extends { ticker: string }>(
  rows: T[],
  ticker: string,
): T | null {
  return rows.find((row) => row.ticker === ticker) ?? null;
}
