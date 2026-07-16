import { useState, type FormEvent } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import type { AppError } from "../types";
import Field from "../components/Field";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";

const PROVIDER_LABELS: Record<string, string> = {
  gemini: "Gemini",
  claude: "Claude",
  openai: "OpenAI",
};

const KEY_PLACEHOLDERS: Record<string, string> = {
  gemini: "AIza...",
  claude: "sk-ant-...",
  openai: "sk-...",
};

function ApiKeyGate({ provider }: { provider: string }) {
  const [key, setKey] = useState("");
  const queryClient = useQueryClient();
  const providerLabel = PROVIDER_LABELS[provider] ?? provider;

  const storeKeyMutation = useMutation<void, AppError, string>({
    mutationFn: (key) => invoke("store_api_key", { provider, key }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["api-key-status", provider] });
      setKey("");
    },
  });

  function handleSubmit(event: FormEvent) {
    event.preventDefault();
    storeKeyMutation.mutate(key.trim());
  }

  return (
    <form onSubmit={handleSubmit} className="flex flex-col gap-4 p-4">
      <p className="text-sm text-muted-foreground">
        Cole sua chave de API do {providerLabel} pra habilitar o chat. Ela fica
        guardada só no keyring do seu sistema, nunca no banco ou no git.
      </p>

      <Field label={`Chave de API do ${providerLabel}`}>
        <Input
          type="password"
          value={key}
          onChange={(e) => setKey(e.currentTarget.value)}
          placeholder={KEY_PLACEHOLDERS[provider] ?? ""}
        />
      </Field>

      {storeKeyMutation.isError && (
        <p className="text-red-600">{storeKeyMutation.error.message}</p>
      )}

      <Button
        type="submit"
        disabled={!key.trim() || storeKeyMutation.isPending}
        className="w-fit"
      >
        {storeKeyMutation.isPending ? "Salvando..." : "Salvar chave"}
      </Button>
    </form>
  );
}

export default ApiKeyGate;
