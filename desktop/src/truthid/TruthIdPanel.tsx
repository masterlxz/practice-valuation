import { invoke } from "@tauri-apps/api/core";
import { useMutation } from "@tanstack/react-query";
import type { AppError } from "../types";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

type TruthIdHandshakeResult = {
  port: number;
  desktop_version: string;
  accepted: boolean;
};

type TruthIdSignResult = {
  status: string;
  user_op_hash: string | null;
  transaction_hash: string | null;
  error: string | null;
};

/**
 * Prova de conceito mínima da fatia 3: descobre um TruthID Desktop rodando
 * na mesma máquina, faz handshake, e manda 1 pedido de assinatura de teste
 * (transferência de valor zero pro endereço de burn — sem efeito econômico,
 * mas passa pelo pipeline real de UserOperation). Não é a Fase 8 (sync via
 * IPFS) — só prova que o mecanismo de assinatura delegada funciona.
 */
function TruthIdPanel() {
  const connectionMutation = useMutation<TruthIdHandshakeResult, AppError, void>({
    mutationFn: () => invoke("test_truthid_connection"),
  });

  const signRequestMutation = useMutation<TruthIdSignResult, AppError, void>({
    mutationFn: () => invoke("send_test_sign_request"),
  });

  return (
    <Card>
      <CardHeader>
        <CardTitle>TruthID Sync (proof of concept)</CardTitle>
      </CardHeader>
      <CardContent className="flex flex-col gap-6">
        <div className="flex flex-col gap-2">
          <Button
            onClick={() => connectionMutation.mutate()}
            disabled={connectionMutation.isPending}
          >
            {connectionMutation.isPending ? "Looking for TruthID..." : "Test connection"}
          </Button>
          {connectionMutation.isError && (
            <p className="text-red-600">{connectionMutation.error.message}</p>
          )}
          {connectionMutation.isSuccess && (
            <p className="text-green-700">
              Found TruthID Desktop {connectionMutation.data.desktop_version} on port{" "}
              {connectionMutation.data.port}.
            </p>
          )}
        </div>

        <div className="flex flex-col gap-2">
          <Button
            variant="outline"
            onClick={() => signRequestMutation.mutate()}
            disabled={signRequestMutation.isPending}
          >
            {signRequestMutation.isPending
              ? "Waiting for approval in TruthID..."
              : "Send test sign-request"}
          </Button>
          {signRequestMutation.isError && (
            <p className="text-red-600">{signRequestMutation.error.message}</p>
          )}
          {signRequestMutation.isSuccess && (
            <div>
              <p>Status: {signRequestMutation.data.status}</p>
              {signRequestMutation.data.user_op_hash && (
                <p className="break-all">userOpHash: {signRequestMutation.data.user_op_hash}</p>
              )}
              {signRequestMutation.data.transaction_hash && (
                <p className="break-all">
                  transactionHash: {signRequestMutation.data.transaction_hash}
                </p>
              )}
              {signRequestMutation.data.error && (
                <p className="text-red-600">{signRequestMutation.data.error}</p>
              )}
            </div>
          )}
        </div>
      </CardContent>
    </Card>
  );
}

export default TruthIdPanel;
