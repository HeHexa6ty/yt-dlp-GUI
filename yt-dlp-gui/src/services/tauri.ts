import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type {
  DownloadProgress,
  DownloadRequest,
  DownloadResult,
  FfmpegStatus,
  MediaInfo,
} from "../types";

/** Event name used by the backend for download progress updates. */
const DOWNLOAD_PROGRESS_EVENT = "download-progress";

/** Error object serialized by the Rust `AppError` type. */
export interface BackendError {
  code: string;
  message: string;
}
/**
 * Type guard to check if a given value is a `BackendError`.
 * @returns True if the value is a `BackendError`, otherwise false.
 */
function isBackendError(value: unknown): value is BackendError {
  return (
    typeof value === "object" &&
    value !== null &&
    "message" in value &&
    typeof (value as { message?: unknown }).message === "string" &&
    "code" in value &&
    typeof (value as { code?: unknown }).code === "string"
  );
}

/** Extracts a readable message from a rejected `invoke` call. */
export function getErrorMessage(error: unknown): string {
  if (typeof error === "string") {
    return error;
  }

  if (isBackendError(error)) {
    return error.message;
  }

  return "Something went wrong while talking to the backend.";
}

/**
 * Returns the structured backend error when there is one, so callers can react
 * to a stable `code` (for example `file-exists`).
 */
export function getBackendError(error: unknown): BackendError | null {
  return isBackendError(error) ? error : null;
}

/** Asks the Rust backend to analyze a URL and return preview metadata. */
export async function analyzeUrl(url: string): Promise<MediaInfo> {
  return invoke<MediaInfo>("analyze_url", { url });
}

/** Starts a download; the backend keeps the file naming and conflict rules. */
export async function downloadMedia(
  request: DownloadRequest,
): Promise<DownloadResult> {
  return invoke<DownloadResult>("download_media", { request });
}

/** Subscribes to backend progress events. Returns an unsubscribe function. */
export function onDownloadProgress(
  handler: (progress: DownloadProgress) => void,
): Promise<UnlistenFn> {
  return listen<DownloadProgress>(DOWNLOAD_PROGRESS_EVENT, (event) => {
    handler(event.payload);
  });
}

/** Asks the backend to detect FFmpeg and ffprobe and report install options. */
export async function getFfmpegStatus(): Promise<FfmpegStatus> {
  return invoke<FfmpegStatus>("ffmpeg_status");
}

/** Validates and saves a user-selected `ffmpeg` binary. */
export async function setFfmpegPath(path: string): Promise<FfmpegStatus> {
  return invoke<FfmpegStatus>("ffmpeg_set_path", { path });
}

/** Forgets the saved path and returns to automatic detection. */
export async function clearFfmpegPath(): Promise<FfmpegStatus> {
  return invoke<FfmpegStatus>("ffmpeg_clear_path");
}

/** Runs the selected platform package manager, then re-runs detection. */
export async function installFfmpeg(installer: string): Promise<FfmpegStatus> {
  return invoke<FfmpegStatus>("ffmpeg_install", { installer });
}
