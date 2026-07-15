import { useState } from "react";
import BazinForm from "./models/BazinForm";
import GrahamForm from "./models/GrahamForm";
import GordonForm from "./models/GordonForm";
import DcfForm from "./models/DcfForm";
import BanksForm from "./models/BanksForm";
import RimForm from "./models/RimForm";
import RnavForm from "./models/RnavForm";
import ProjectedCeilingForm from "./models/ProjectedCeilingForm";
import CryptoScorePanel from "./crypto/CryptoScorePanel";
import SavedValuationsPanel from "./valuations/SavedValuationsPanel";
import StockLookupPanel from "./stock-lookup/StockLookupPanel";
import AlertsPanel from "./alerts/AlertsPanel";
import TruthIdPanel from "./truthid/TruthIdPanel";
import ChatPanel from "./chat/ChatPanel";
import ChatToggleButton from "./chat/ChatToggleButton";
import type { GeminiContent } from "./chat/types";
import Field from "./components/Field";
import { Button } from "@/components/ui/button";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";

const MODELS = {
  bazin: { label: "Bazin", component: BazinForm },
  graham: { label: "Graham", component: GrahamForm },
  gordon: { label: "Gordon / DDM", component: GordonForm },
  dcf: { label: "DCF / FCFF", component: DcfForm },
  banks: { label: "Banks (P/B)", component: BanksForm },
  rim: { label: "RIM (Bancos)", component: RimForm },
  rnav: { label: "RNAV", component: RnavForm },
  projected_ceiling: {
    label: "Projected Ceiling",
    component: ProjectedCeilingForm,
  },
} as const;

type ModelKey = keyof typeof MODELS;

const SECTIONS = {
  valuation: "Valuation",
  lookup: "Stock Lookup",
  crypto: "Crypto Score",
  alerts: "Alerts",
  truthid: "TruthID Sync",
} as const;

type ValuationView = "form" | "saved";

type SectionKey = keyof typeof SECTIONS;

function App() {
  const [section, setSection] = useState<SectionKey>("valuation");
  const [valuationView, setValuationView] = useState<ValuationView>("form");
  const [selectedModel, setSelectedModel] = useState<ModelKey>("bazin");
  const SelectedForm = MODELS[selectedModel].component;
  const [chatOpen, setChatOpen] = useState(false);
  const [chatHistory, setChatHistory] = useState<GeminiContent[]>([]);

  return (
    <>
      <main className="mx-auto max-w-6xl p-8">
        <Tabs
          value={section}
          onValueChange={(value) => setSection(value as SectionKey)}
        >
          <TabsList className="mb-6">
            {Object.entries(SECTIONS).map(([key, label]) => (
              <TabsTrigger key={key} value={key}>
                {label}
              </TabsTrigger>
            ))}
          </TabsList>

          <TabsContent value="valuation" className="flex flex-col gap-6">
            {valuationView === "form" ? (
              <>
                <div className="flex items-end justify-between gap-4">
                  <Field label="Valuation model" className="flex-1">
                    <Select
                      value={selectedModel}
                      onValueChange={(value) => setSelectedModel(value as ModelKey)}
                    >
                      <SelectTrigger className="w-full">
                        <SelectValue />
                      </SelectTrigger>
                      <SelectContent>
                        {Object.entries(MODELS).map(([key, { label }]) => (
                          <SelectItem key={key} value={key}>
                            {label}
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                  </Field>
                  <Button
                    type="button"
                    variant="outline"
                    onClick={() => setValuationView("saved")}
                  >
                    Saved Valuations
                  </Button>
                </div>

                <SelectedForm />
              </>
            ) : (
              <>
                <Button
                  type="button"
                  variant="outline"
                  className="w-fit"
                  onClick={() => setValuationView("form")}
                >
                  ← Back
                </Button>
                <SavedValuationsPanel />
              </>
            )}
          </TabsContent>

          <TabsContent value="lookup">
            <StockLookupPanel />
          </TabsContent>

          <TabsContent value="crypto">
            <CryptoScorePanel />
          </TabsContent>

          <TabsContent value="alerts">
            <AlertsPanel />
          </TabsContent>

          <TabsContent value="truthid">
            <TruthIdPanel />
          </TabsContent>
        </Tabs>
      </main>
      <ChatToggleButton open={chatOpen} onToggle={() => setChatOpen((o) => !o)} />
      <ChatPanel
        open={chatOpen}
        onOpenChange={setChatOpen}
        history={chatHistory}
        onHistoryChange={setChatHistory}
      />
    </>
  );
}

export default App;
