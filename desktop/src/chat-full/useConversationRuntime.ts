import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useExternalStoreRuntime } from "@assistant-ui/react";
import type { AppendMessage, ThreadMessageLike } from "@assistant-ui/react";
import type { AppError } from "../types";
import type { ApiKeySummary } from "../settings/SettingsPage";

export type ConversationMessage = {
  id: number;
  role: string;
  content: string;
  created_at: string;
  input_tokens: number | null;
  output_tokens: number | null;
};

// Placeholder id for the user's message while `send_conversation_message` is
// in flight — only one can exist at a time (sending is disabled meanwhile),
// replaced by the real row once the mutation settles and the query refetches.
const OPTIMISTIC_USER_MESSAGE_ID = -1;

// Mesmo tier barato/rápido por provider já usado em `ChatPanel.tsx` (7.9.5) —
// só sugestão de modelo default, o campo continua livre pra editar.
const DEFAULT_MODEL_BY_PROVIDER: Record<string, string> = {
  gemini: "gemini-3.1-flash-lite",
  claude: "claude-haiku-4-5",
  openai: "gpt-5-mini",
};

function convertMessage(message: ConversationMessage): ThreadMessageLike {
  const isAssistant = message.role === "model";
  return {
    id: String(message.id),
    role: isAssistant ? "assistant" : "user",
    content: message.content,
    metadata:
      isAssistant && message.input_tokens !== null && message.output_tokens !== null
        ? {
            custom: {
              inputTokens: message.input_tokens,
              outputTokens: message.output_tokens,
            },
          }
        : undefined,
  };
}

function extractText(message: AppendMessage): string {
  const textPart = message.content.find(
    (part): part is { type: "text"; text: string } => part.type === "text",
  );
  return textPart?.text ?? "";
}

export function useConversationRuntime(conversationId: number | null) {
  const queryClient = useQueryClient();
  const [selectedKeyId, setSelectedKeyId] = useState<number | null>(null);
  const [model, setModel] = useState("");
  const [error, setError] = useState<string | null>(null);

  const keysQuery = useQuery<ApiKeySummary[], AppError>({
    queryKey: ["api-keys"],
    queryFn: () => invoke("list_api_keys"),
  });
  const keys = keysQuery.data ?? [];

  const messagesQuery = useQuery<ConversationMessage[], AppError>({
    queryKey: ["conversation-messages", conversationId],
    queryFn: () => invoke("get_conversation_messages", { id: conversationId }),
    enabled: conversationId !== null,
  });

  // Same auto-select-first-key pattern as `ChatPanel.tsx`.
  useEffect(() => {
    if (keys.length === 0) {
      setSelectedKeyId(null);
      return;
    }
    if (!keys.some((k) => k.id === selectedKeyId)) {
      const first = keys[0];
      setSelectedKeyId(first.id);
      setModel(DEFAULT_MODEL_BY_PROVIDER[first.provider] ?? "");
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [keys]);

  function handleKeyChange(nextKeyId: string) {
    const id = Number(nextKeyId);
    setSelectedKeyId(id);
    const key = keys.find((k) => k.id === id);
    if (key) setModel(DEFAULT_MODEL_BY_PROVIDER[key.provider] ?? "");
  }

  const sendMutation = useMutation<ConversationMessage, AppError, string>({
    mutationFn: (content) =>
      invoke("send_conversation_message", {
        conversationId,
        keyId: selectedKeyId,
        model,
        content,
      }),
  });

  const messages = messagesQuery.data ?? [];
  const totalTokens = messages.reduce(
    (sum, m) => sum + (m.input_tokens ?? 0) + (m.output_tokens ?? 0),
    0,
  );

  const runtime = useExternalStoreRuntime({
    messages,
    isRunning: sendMutation.isPending,
    convertMessage,
    onNew: async (message) => {
      if (conversationId === null || selectedKeyId === null) return;
      const text = extractText(message);
      if (!text) return;

      setError(null);
      queryClient.setQueryData<ConversationMessage[]>(
        ["conversation-messages", conversationId],
        (prev) => [
          ...(prev ?? []),
          {
            id: OPTIMISTIC_USER_MESSAGE_ID,
            role: "user",
            content: text,
            created_at: new Date().toISOString(),
            input_tokens: null,
            output_tokens: null,
          },
        ],
      );

      try {
        await sendMutation.mutateAsync(text);
      } catch (err) {
        setError((err as AppError).message);
      } finally {
        // `send_conversation_message` persists the user message even when
        // the provider call fails afterwards — refetch either way so the
        // optimistic placeholder is replaced by the real row(s).
        queryClient.invalidateQueries({ queryKey: ["conversation-messages", conversationId] });
        queryClient.invalidateQueries({ queryKey: ["conversations"] });
      }
    },
  });

  return {
    runtime,
    keys,
    selectedKeyId,
    model,
    setModel,
    handleKeyChange,
    hasKey: keys.length > 0,
    error,
    totalTokens,
  };
}
