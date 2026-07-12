import { useEffect, useRef, useState, type KeyboardEvent } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useMutation, useQuery } from "@tanstack/react-query";
import type { AppError } from "../types";
import type { GeminiContent } from "./types";
import ApiKeyGate from "./ApiKeyGate";
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
} from "@/components/ui/sheet";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { SendIcon } from "lucide-react";

function MessageBubble({ message }: { message: GeminiContent }) {
  const isUser = message.role === "user";
  return (
    <div className={`flex ${isUser ? "justify-end" : "justify-start"}`}>
      <div
        className={`max-w-[85%] rounded-lg px-3 py-2 text-sm whitespace-pre-wrap ${
          isUser
            ? "bg-primary text-primary-foreground"
            : "bg-muted text-foreground"
        }`}
      >
        {message.parts.map((part) => part.text).join("")}
      </div>
    </div>
  );
}

function ChatPanel({
  open,
  onOpenChange,
  history,
  onHistoryChange,
}: {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  history: GeminiContent[];
  onHistoryChange: (history: GeminiContent[]) => void;
}) {
  const [input, setInput] = useState("");
  const scrollRef = useRef<HTMLDivElement>(null);

  const keyStatusQuery = useQuery<boolean, AppError>({
    queryKey: ["api-key-status", "gemini"],
    queryFn: () => invoke("get_api_key_status", { provider: "gemini" }),
    enabled: open,
  });

  const sendMutation = useMutation<string, AppError, GeminiContent[]>({
    mutationFn: (nextHistory) =>
      invoke("ask_ai", { provider: "gemini", history: nextHistory }),
  });

  useEffect(() => {
    scrollRef.current?.scrollTo({ top: scrollRef.current.scrollHeight });
  }, [history, sendMutation.isPending]);

  function handleSend() {
    const text = input.trim();
    if (!text || sendMutation.isPending) return;

    const nextHistory: GeminiContent[] = [
      ...history,
      { role: "user", parts: [{ text }] },
    ];
    onHistoryChange(nextHistory);
    setInput("");

    sendMutation.mutate(nextHistory, {
      onSuccess: (reply) => {
        onHistoryChange([...nextHistory, { role: "model", parts: [{ text: reply }] }]);
      },
    });
  }

  function handleKeyDown(event: KeyboardEvent<HTMLInputElement>) {
    if (event.key === "Enter" && !event.shiftKey) {
      event.preventDefault();
      handleSend();
    }
  }

  const hasKey = keyStatusQuery.data === true;

  return (
    <Sheet open={open} onOpenChange={onOpenChange}>
      <SheetContent side="right" className="flex w-full flex-col sm:max-w-md">
        <SheetHeader>
          <SheetTitle>Chat de IA</SheetTitle>
        </SheetHeader>

        {keyStatusQuery.isPending ? null : !hasKey ? (
          <ApiKeyGate />
        ) : (
          <>
            <div
              ref={scrollRef}
              className="flex flex-1 flex-col gap-3 overflow-y-auto px-4"
            >
              {history.length === 0 && (
                <p className="text-sm text-muted-foreground">
                  Pergunte sobre suas valuations salvas ou alertas.
                </p>
              )}
              {history.map((message, index) => (
                <MessageBubble key={index} message={message} />
              ))}
              {sendMutation.isPending && (
                <div className="flex justify-start">
                  <div className="max-w-[85%] rounded-lg bg-muted px-3 py-2 text-sm text-muted-foreground">
                    Pensando...
                  </div>
                </div>
              )}
              {sendMutation.isError && (
                <p className="text-red-600">{sendMutation.error.message}</p>
              )}
            </div>

            <div className="flex gap-2 p-4">
              <Input
                value={input}
                onChange={(e) => setInput(e.currentTarget.value)}
                onKeyDown={handleKeyDown}
                disabled={sendMutation.isPending}
                placeholder="Digite sua pergunta..."
              />
              <Button
                type="button"
                size="icon"
                onClick={handleSend}
                disabled={!input.trim() || sendMutation.isPending}
              >
                <SendIcon />
              </Button>
            </div>
          </>
        )}
      </SheetContent>
    </Sheet>
  );
}

export default ChatPanel;
