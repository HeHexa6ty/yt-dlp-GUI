import { useState } from "react";
import FfmpegTab from "./components/FfmpegTab";
import YtDlpTab from "./components/YtDlpTab";
import "./App.css";

type Tab = "yt-dlp" | "ffmpeg";

const TABS: ReadonlyArray<{ id: Tab; label: string }> = [
  { id: "yt-dlp", label: "yt-dlp" },
  { id: "ffmpeg", label: "FFmpeg" },
];

function App() {
  const [tab, setTab] = useState<Tab>("yt-dlp");

  return (
    <main className="container">
      <header className="app-header">
        <h1>yt-dlp GUI</h1>
        <nav className="tabs" role="tablist" aria-label="Application sections">
          {TABS.map(({ id, label }) => (
            <button
              key={id}
              type="button"
              role="tab"
              id={`tab-${id}`}
              aria-selected={tab === id}
              aria-controls={`panel-${id}`}
              className={`tab-button${tab === id ? " tab-button--active" : ""}`}
              onClick={() => setTab(id)}
            >
              {label}
            </button>
          ))}
        </nav>
      </header>

      {/* Both panels stay mounted so switching tabs keeps the current download
          analysis and the FFmpeg detection result. */}
      <section
        role="tabpanel"
        id="panel-yt-dlp"
        aria-labelledby="tab-yt-dlp"
        hidden={tab !== "yt-dlp"}
      >
        <YtDlpTab />
      </section>

      <section
        role="tabpanel"
        id="panel-ffmpeg"
        aria-labelledby="tab-ffmpeg"
        hidden={tab !== "ffmpeg"}
      >
        <FfmpegTab />
      </section>
    </main>
  );
}

export default App;
