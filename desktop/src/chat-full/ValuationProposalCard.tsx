import type { ToolCallMessagePartProps } from "@assistant-ui/react";
import { Card, CardContent } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import VerdictBadge from "../components/VerdictBadge";
import type { ProposeValuationArgs, ValuationProposalSummary } from "./useConversationRuntime";

// Fase 7.10.4 — rich preview for the one tool this app declares
// (`propose_valuation`), overriding the library's default raw-JSON-args
// card (`components/assistant-ui/tool-fallback.tsx`) via `<Thread
// components={{ ToolFallback: ValuationProposalCard }} />` in `ChatScreen.tsx`.
// Reuses the same visual pieces as the manual calculator flow
// (`components/ValuationResult.tsx`/`VerdictBadge.tsx`) so an AI-proposed
// valuation reads the same as one entered by hand.
function ValuationProposalCard({
  toolName,
  args,
  approval,
  respondToApproval,
}: ToolCallMessagePartProps<ProposeValuationArgs, ValuationProposalSummary>) {
  if (toolName !== "propose_valuation") return null;

  const isPending = approval?.approved === undefined && approval?.resolution === undefined;
  const isRejected = approval?.approved === false;

  return (
    <Card className="mt-2 border-dashed">
      <CardContent className="flex flex-col gap-2 pt-4">
        <p className="text-sm font-medium">
          Proposta de valuation: {args.ticker} ({args.model}, ano-ref {args.reference_year})
        </p>
        <p>
          Fair price: <strong>R$ {args.preview.fair_price.toFixed(2)}</strong>
        </p>
        <p>
          Safety margin: <strong>{(args.preview.safety_margin * 100).toFixed(1)}%</strong>
        </p>
        <p className="flex items-center gap-2">
          Verdict: <VerdictBadge verdict={args.preview.verdict} />
        </p>

        {isPending && (
          <div className="flex gap-2 pt-2">
            <Button size="sm" onClick={() => respondToApproval({ approved: true })}>
              Criar valuation
            </Button>
            <Button size="sm" variant="outline" onClick={() => respondToApproval({ approved: false })}>
              Descartar
            </Button>
          </div>
        )}
        {isRejected && <p className="text-muted-foreground text-xs">Descartada.</p>}
      </CardContent>
    </Card>
  );
}

export default ValuationProposalCard;
