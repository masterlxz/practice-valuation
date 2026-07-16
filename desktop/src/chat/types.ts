// Mirrors GeminiPart/GeminiContent in desktop/src-tauri/src/commands/chat.rs.
// Shared history shape sent to `ask_ai` regardless of provider — the backend
// converts it into each provider's own message format (Claude/OpenAI use
// "assistant" instead of "model" for the reply role).
export type GeminiPart = {
  text: string;
};

export type GeminiRole = "user" | "model";

export type GeminiContent = {
  role: GeminiRole;
  parts: GeminiPart[];
};
