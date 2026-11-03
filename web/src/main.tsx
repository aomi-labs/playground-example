import { AomiWidget } from "@aomi-labs/widget";
import "@aomi-labs/widget/styles.css";
import { createRoot } from "react-dom/client";

const applicationId = Number(import.meta.env.VITE_AOMI_APPLICATION_ID?.trim());
// Leave baseUrl unset to use the widget's default host.
const baseUrl = import.meta.env.VITE_AOMI_BASE_URL?.trim() || undefined;

console.log(`aomi host: ${baseUrl ? new URL(baseUrl).host : "widget default"}  app: ${applicationId}`);

function App() {
  if (!Number.isSafeInteger(applicationId) || applicationId <= 0) {
    return <p>Set VITE_AOMI_APPLICATION_ID in web/.env (see .env.example).</p>;
  }
  return (
    <AomiWidget
      applicationId={applicationId}
      baseUrl={baseUrl}
      // Direct routing sends every message to this app instead of the Auto agent.
      routing={{
        targets: [{ mode: "direct", apps: [{ applicationId }] }],
        defaultMode: "direct",
      }}
      height="100vh"
    />
  );
}

createRoot(document.getElementById("root")!).render(<App />);
