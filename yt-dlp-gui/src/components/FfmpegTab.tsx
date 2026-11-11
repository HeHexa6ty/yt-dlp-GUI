import { useCallback, useEffect, useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { openUrl } from "@tauri-apps/plugin-opener";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import {
  clearFfmpegPath,
  getErrorMessage,
  getFfmpegStatus,
  installFfmpeg,
  setFfmpegPath,
} from "../services/tauri";
import type { FfmpegStatus } from "../types";
import "./FfmpegTab.css";

const SOURCE_LABELS: Record<string, string> = {
  environment: "FFMPEG_PATH environment variable",
  setting: "Saved in this app",
  PATH: "System PATH",
  system: "Common install location",
};

/** Human-readable package manager name for the "Copy command" button. */
function installerName(label: string): string {
  return label.split(" (")[0];
}

export default function FfmpegTab() {
  const [status, setStatus] = useState<FfmpegStatus | null>(null);
  const [loading, setLoading] = useState(true);
  const [busy, setBusy] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [notice, setNotice] = useState<string | null>(null);

  const refresh = useCallback(async () => {
    setLoading(true);
    try {
      setStatus(await getFfmpegStatus());
      setError(null);
    } catch (caught) {
      setError(getErrorMessage(caught));
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    void refresh();
  }, [refresh]);

  const chooseBinary = async () => {
    setError(null);
    setNotice(null);

    try {
      const selected = await open({
        directory: false,
        multiple: false,
        title: "Select the ffmpeg executable",
        filters:
          status?.platform === "windows"
            ? [{ name: "Executable", extensions: ["exe"] }]
            : undefined,
      });

      if (typeof selected !== "string" || selected === "") {
        return;
      }

      setBusy("choose");
      const next = await setFfmpegPath(selected);
      setStatus(next);
      setNotice(
        next.path
          ? `Using FFmpeg at ${next.path}`
          : "The selected file was saved, but FFmpeg could not be found.",
      );
    } catch (caught) {
      setError(getErrorMessage(caught));
    } finally {
      setBusy(null);
    }
  };

  const useAutomatic = async () => {
    setError(null);
    setNotice(null);
    setBusy("clear");

    try {
      const next = await clearFfmpegPath();
      setStatus(next);
      setNotice(
        next.path
          ? `Using FFmpeg at ${next.path}`
          : "Saved path cleared. FFmpeg is still not installed.",
      );
    } catch (caught) {
      setError(getErrorMessage(caught));
    } finally {
      setBusy(null);
    }
  };

  const install = async (id: string, label: string) => {
    setError(null);
    setNotice(null);
    setBusy(id);

    try {
      const next = await installFfmpeg(id);
      setStatus(next);
      setNotice(
        next.path
          ? `${installerName(label)} finished. Using FFmpeg at ${next.path}`
          : `${installerName(label)} finished, but FFmpeg was not found yet. Restart the app or choose the binary manually.`,
      );
    } catch (caught) {
      setError(getErrorMessage(caught));
    } finally {
      setBusy(null);
    }
  };

  const copyCommand = async (command: string) => {
    try {
      await writeText(command);
      setNotice(`Copied: ${command}`);
    } catch (caught) {
      setError(getErrorMessage(caught));
    }
  };

  const openDownloadPage = async () => {
    try {
      await openUrl(status?.downloadUrl ?? "https://ffmpeg.org/download.html");
    } catch (caught) {
      setError(getErrorMessage(caught));
    }
  };

  const found = Boolean(status?.path);
  const sourceLabel = status?.source ? SOURCE_LABELS[status.source] : null;
  // Show the package managers that are actually usable; fall back to the full
  // list so the user can still copy a command when none is detected.
  const availableInstallers =
    status?.installers.filter((installer) => installer.available) ?? [];
  const visibleInstallers =
    availableInstallers.length > 0
      ? availableInstallers
      : (status?.installers ?? []);

  return (
    <div className="ffmpeg-tab">
      <div>
        <h2>FFmpeg</h2>
        <p className="ffmpeg-intro">
          FFmpeg merges video and audio, extracts MP3, and converts GIF and image
          downloads. It is not bundled with this app, so it has to be installed
          on your system or selected manually.
        </p>
      </div>

      <section className="ffmpeg-card" aria-live="polite">
        <div className="ffmpeg-status-head">
          <span className={`ffmpeg-badge ${found ? "ffmpeg-badge--ok" : "ffmpeg-badge--missing"}`}>
            {loading && !status
              ? "Checking…"
              : found
                ? "FFmpeg found"
                : "FFmpeg missing"}
          </span>
          <span className="ffmpeg-status-summary">
            {found
              ? `Version ${status?.version ?? "unknown"}`
              : "Downloads that need conversion will fail until FFmpeg is available."}
          </span>
          <span className="ffmpeg-spacer" />
          <button
            type="button"
            className="ffmpeg-button"
            onClick={() => {
              void refresh();
            }}
            disabled={loading || busy !== null}
          >
            {loading ? "Checking…" : "Check again"}
          </button>
        </div>

        {found ? (
          <dl className="ffmpeg-details">
            <dt>Binary</dt>
            <dd title={status?.path ?? undefined}>{status?.path}</dd>
            {sourceLabel ? (
              <>
                <dt>Source</dt>
                <dd>{sourceLabel}</dd>
              </>
            ) : null}
            <dt>ffprobe</dt>
            <dd>
              {status?.ffprobePath
                ? `${status.ffprobePath}${
                    status.ffprobeVersion ? ` (${status.ffprobeVersion})` : ""
                  }`
                : "Not found next to ffmpeg or on PATH. Duration probing may be unavailable."}
            </dd>
            <dt>MP3 encoder</dt>
            <dd>
              {status?.hasLibmp3lame
                ? "libmp3lame is available."
                : "libmp3lame is missing. MP3 downloads will fail; use a full FFmpeg build."}
            </dd>
          </dl>
        ) : (
          <p className="ffmpeg-empty">
            Install FFmpeg with a package manager below, or point the app at an
            existing <code>ffmpeg</code> binary. You can also download a build
            from the official FFmpeg website.
          </p>
        )}

        {status?.versionLine ? (
          <p className="ffmpeg-version-line" title={status.versionLine}>
            {status.versionLine}
          </p>
        ) : null}
      </section>

      <div className="ffmpeg-actions">
        <button
          type="button"
          className="ffmpeg-button"
          onClick={() => {
            void chooseBinary();
          }}
          disabled={busy !== null}
        >
          Choose existing binary…
        </button>
        <button
          type="button"
          className="ffmpeg-button"
          onClick={() => {
            void useAutomatic();
          }}
          disabled={busy !== null || (status?.source !== "setting" && !found)}
          title="Forget the saved binary and search FFMPEG_PATH, PATH, and common locations"
        >
          Use automatic detection
        </button>
        <button
          type="button"
          className="ffmpeg-button"
          onClick={() => {
            void openDownloadPage();
          }}
        >
          Open download page
        </button>
      </div>

      <section className="ffmpeg-card">
        <h3>Install FFmpeg</h3>
        {visibleInstallers.length ? (
          <ul className="ffmpeg-installers">
            {visibleInstallers.map((installer) => (
              <li key={installer.id} className="ffmpeg-installer">
                <div className="ffmpeg-installer-text">
                  <span className="ffmpeg-installer-label">{installer.label}</span>
                  <code className="ffmpeg-installer-command">
                    {installer.command}
                  </code>
                  {!installer.available ? (
                    <span className="ffmpeg-installer-note">
                      Not available on this system
                    </span>
                  ) : null}
                </div>
                <div className="ffmpeg-installer-actions">
                  <button
                    type="button"
                    className="ffmpeg-button ffmpeg-button--small"
                    onClick={() => {
                      void copyCommand(installer.command);
                    }}
                  >
                    Copy
                  </button>
                  <button
                    type="button"
                    className="ffmpeg-button ffmpeg-button--primary ffmpeg-button--small"
                    onClick={() => {
                      void install(installer.id, installer.label);
                    }}
                    disabled={
                      !installer.available || busy !== null || loading
                    }
                  >
                    {busy === installer.id ? "Installing…" : "Install"}
                  </button>
                </div>
              </li>
            ))}
          </ul>
        ) : (
          <p className="ffmpeg-empty">
            No supported package manager was detected. Use the download page or
            choose an existing binary.
          </p>
        )}
        <p className="ffmpeg-hint">
          Linux installations ask for administrator access through{" "}
          <code>pkexec</code>. If that is unavailable, copy the command and run
          it in a terminal.
        </p>
      </section>

      {notice ? (
        <p className="ffmpeg-notice" role="status">
          {notice}
        </p>
      ) : null}
      {error ? (
        <p className="ffmpeg-error" role="alert">
          {error}
        </p>
      ) : null}
    </div>
  );
}
