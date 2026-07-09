import type { ReactNode } from "react";
import { Label } from "@/components/ui/label";

function Field({ label, children }: { label: string; children: ReactNode }) {
  return (
    <div className="grid gap-1.5">
      <Label>{label}</Label>
      {children}
    </div>
  );
}

export default Field;
