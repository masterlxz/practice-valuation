import { useState, type FormEvent } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import type { AppError } from "../types";
import Field from "../components/Field";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";

function ApiKeyGate() {
  const [key, setKey] = useState("");
  const queryClient = useQueryClient();

  const storeKeyMutation = useMutation<void, AppError, string>({
    mutationFn: (key) => invoke("store_api_key", { provider: "gemini", key }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["api-key-status", "gemini"] });
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
        Cole sua chave de API do Gemini pra habilitar o chat. Ela fica guardada só
        no keyring do seu sistema, nunca no banco ou no git.
      </p>

      <Field label="Chave de API do Gemini">
        <Input
          type="password"
          value={key}
          onChange={(e) => setKey(e.currentTarget.value)}
          placeholder="AIza..."
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
