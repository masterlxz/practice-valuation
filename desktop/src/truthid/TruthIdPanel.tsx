import { useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useMutation } from "@tanstack/react-query";
import type { AppError } from "../types";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { renderQrToCanvas } from "./renderQr";

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

type CrossDeviceSession = {
  session_id: string;
  ephemeral_priv_key_hex: string;
  expires_at_ms: number;
  qr_payload_json: string;
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

  // Fatia cross-device: em vez de falar loopback com um TruthID Desktop na
  // mesma máquina, gera um QR e varre a LAN esperando um celular pareado
  // responder — mesmo protocolo `/sign-request` cross-device que o TruthID
  // Mobile já implementa desde a Sessão 110 (LAN, portas 48050-54).
  const crossDeviceSessionMutation = useMutation<CrossDeviceSession, AppError, void>({
    mutationFn: () => invoke("create_cross_device_sign_request"),
  });

  const crossDeviceResultMutation = useMutation<TruthIdSignResult, AppError, CrossDeviceSession>({
    mutationFn: (session) =>
      invoke("await_cross_device_sign_request_response", {
        sessionId: session.session_id,
        ephemeralPrivKeyHex: session.ephemeral_priv_key_hex,
        expiresAtMs: session.expires_at_ms,
      }),
  });

  const qrCanvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    const session = crossDeviceSessionMutation.data;
    if (!session || !qrCanvasRef.current) return;
    void renderQrToCanvas(qrCanvasRef.current, session.qr_payload_json);
    // O celular só começa a servir depois de aprovar (e, no sign-request,
    // depois da UserOp terminar de executar, até ~60s) — dispara a
    // varredura assim que o QR aparece, sem esperar clique, mesma filosofia
    // de "já começa a servir" do lado Mobile.
    crossDeviceResultMutation.mutate(session);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [crossDeviceSessionMutation.data]);

  function startCrossDeviceRequest() {
    crossDeviceResultMutation.reset();
    crossDeviceSessionMutation.mutate();
  }

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

        <div className="flex flex-col gap-2 border-t pt-6">
          <p className="text-sm text-muted-foreground">
            Cross-device: scan with your paired TruthID phone instead of a TruthID Desktop on this
            same machine.
          </p>
          <Button
            variant="outline"
            onClick={startCrossDeviceRequest}
            disabled={crossDeviceSessionMutation.isPending || crossDeviceResultMutation.isPending}
          >
            {crossDeviceResultMutation.isPending
              ? "Waiting for your phone (LAN + IPFS backup)..."
              : "Start cross-device request"}
          </Button>
          {crossDeviceSessionMutation.isError && (
            <p className="text-red-600">{crossDeviceSessionMutation.error.message}</p>
          )}
          {crossDeviceSessionMutation.isSuccess && !crossDeviceResultMutation.isSuccess && (
            <div className="flex flex-col items-center gap-2">
              <canvas ref={qrCanvasRef} />
              <p className="text-sm text-muted-foreground">
                Scan this QR with your paired TruthID phone.
              </p>
            </div>
          )}
          {crossDeviceResultMutation.isError && (
            <p className="text-red-600">{crossDeviceResultMutation.error.message}</p>
          )}
          {crossDeviceResultMutation.isSuccess && (
            <div>
              <p>Status: {crossDeviceResultMutation.data.status}</p>
              {crossDeviceResultMutation.data.user_op_hash && (
                <p className="break-all">
                  userOpHash: {crossDeviceResultMutation.data.user_op_hash}
                </p>
              )}
              {crossDeviceResultMutation.data.transaction_hash && (
                <p className="break-all">
                  transactionHash: {crossDeviceResultMutation.data.transaction_hash}
                </p>
              )}
              {crossDeviceResultMutation.data.error && (
                <p className="text-red-600">{crossDeviceResultMutation.data.error}</p>
              )}
            </div>
          )}
        </div>
      </CardContent>
    </Card>
  );
}

export default TruthIdPanel;
