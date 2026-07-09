import { useState } from "react";
import BazinForm from "./models/BazinForm";
import GrahamForm from "./models/GrahamForm";

const MODELS = {
  bazin: { label: "Bazin", component: BazinForm },
  graham: { label: "Graham", component: GrahamForm },
} as const;

type ModelKey = keyof typeof MODELS;

function App() {
  const [selectedModel, setSelectedModel] = useState<ModelKey>("bazin");
  const SelectedForm = MODELS[selectedModel].component;

  return (
    <main className="mx-auto max-w-md p-8">
      <label className="mb-6 flex flex-col gap-1">
        Valuation model
        <select
          value={selectedModel}
          onChange={(e) => setSelectedModel(e.currentTarget.value as ModelKey)}
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
    </main>
  );
}

export default App;
