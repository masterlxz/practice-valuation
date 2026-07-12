import { MessageCircleIcon, XIcon } from "lucide-react";
import { Button } from "@/components/ui/button";

function ChatToggleButton({
  open,
  onToggle,
}: {
  open: boolean;
  onToggle: () => void;
}) {
  return (
    <Button
      type="button"
      size="icon-lg"
      onClick={onToggle}
      className="fixed right-6 bottom-6 z-50 size-12 rounded-full shadow-lg"
    >
      {open ? <XIcon className="size-5" /> : <MessageCircleIcon className="size-5" />}
      <span className="sr-only">{open ? "Fechar chat" : "Abrir chat"}</span>
    </Button>
  );
}

export default ChatToggleButton;
