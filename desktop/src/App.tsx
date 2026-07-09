import { useState } from "react";
import BazinForm from "./models/BazinForm";
import GrahamForm from "./models/GrahamForm";
import GordonForm from "./models/GordonForm";
import DcfForm from "./models/DcfForm";
import BanksForm from "./models/BanksForm";
import RnavForm from "./models/RnavForm";
import ProjectedCeilingForm from "./models/ProjectedCeilingForm";
import CryptoScorePanel from "./crypto/CryptoScorePanel";
import SavedValuationsPanel from "./valuations/SavedValuationsPanel";
import Field from "./components/Field";
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
  rnav: { label: "RNAV", component: RnavForm },
  projected_ceiling: {
    label: "Projected Ceiling",
    component: ProjectedCeilingForm,
  },
} as const;

type ModelKey = keyof typeof MODELS;

const SECTIONS = {
  valuation: "Valuation",
  crypto: "Crypto Score",
  saved: "Saved Valuations",
} as const;

type SectionKey = keyof typeof SECTIONS;

function App() {
  const [section, setSection] = useState<SectionKey>("valuation");
  const [selectedModel, setSelectedModel] = useState<ModelKey>("bazin");
  const SelectedForm = MODELS[selectedModel].component;

  return (
    <main
      className={`mx-auto p-8 ${section === "saved" ? "max-w-4xl" : "max-w-md"}`}
    >
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
          <Field label="Valuation model">
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

          <SelectedForm />
        </TabsContent>

        <TabsContent value="crypto">
          <CryptoScorePanel />
        </TabsContent>

        <TabsContent value="saved">
          <SavedValuationsPanel />
        </TabsContent>
      </Tabs>
    </main>
  );
}

export default App;
