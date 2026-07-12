// Mirrors GeminiPart/GeminiContent in desktop/src-tauri/src/commands/gemini.rs.
export type GeminiPart = {
  text: string;
};

export type GeminiRole = "user" | "model";

export type GeminiContent = {
  role: GeminiRole;
  parts: GeminiPart[];
};
