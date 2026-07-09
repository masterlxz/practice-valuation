import { useState } from "react";
import BazinForm from "./models/BazinForm";
import GrahamForm from "./models/GrahamForm";
import GordonForm from "./models/GordonForm";
import DcfForm from "./models/DcfForm";
import BanksForm from "./models/BanksForm";
import RnavForm from "./models/RnavForm";
import ProjectedCeilingForm from "./models/ProjectedCeilingForm";
import CryptoScorePanel from "./crypto/CryptoScorePanel";

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
} as const;

type SectionKey = keyof typeof SECTIONS;

function App() {
  const [section, setSection] = useState<SectionKey>("valuation");
  const [selectedModel, setSelectedModel] = useState<ModelKey>("bazin");
  const SelectedForm = MODELS[selectedModel].component;

  return (
    <main className="mx-auto max-w-md p-8">
      <div className="mb-6 flex gap-2">
        {Object.entries(SECTIONS).map(([key, label]) => (
          <button
            key={key}
            type="button"
            onClick={() => setSection(key as SectionKey)}
            className={`rounded px-3 py-2 ${
              section === key
                ? "bg-black text-white"
                : "border text-gray-700"
            }`}
          >
            {label}
          </button>
        ))}
      </div>

      {section === "valuation" && (
        <>
          <label className="mb-6 flex flex-col gap-1">
            Valuation model
            <select
              value={selectedModel}
              onChange={(e) =>
                setSelectedModel(e.currentTarget.value as ModelKey)
              }
              className="rounded border px-3 py-2"
            >
              {Object.entries(MODELS).map(([key, { label }]) => (
                <option key={key} value={key}>
                  {label}
                </option>
              ))}
            </select>
          </label>

          <SelectedForm />
        </>
      )}

      {section === "crypto" && <CryptoScorePanel />}
    </main>
  );
}

export default App;
