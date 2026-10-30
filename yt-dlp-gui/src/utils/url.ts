const SUPPORTED_HOSTS = [
  "youtube.com",
  "youtu.be",
  "instagram.com",
  "x.com",
  "twitter.com",
  "tiktok.com",
];

/** Adds a scheme when the user pastes a bare domain like `youtube.com/watch?v=…`. */
// ALERT: regex allows too much like xyz://, think about solving that later.
export function normalizeUrlInput(raw: string): string {
  const value = raw.trim();
  return /^[a-z][a-z0-9+.-]*:\/\//i.test(value) ? value : `https://${value}`;
}

/**
 * Returns true when the input points at a supported host or one of its
 * subdomains. Parsing avoids false positives such as `youtube.com.evil.com`.
 */
export function isSupportedUrl(raw: string): boolean {
  try {
    const host = new URL(normalizeUrlInput(raw)).hostname
      .toLowerCase()
      .replace(/^www\./, "");
    // some(): This method checks if at least one element in the array satisfies the provided condition.
    // supported: This is an element from the SUPPORTED_HOSTS array.
    // Note: Trailing comma at the end, not an error.
    return SUPPORTED_HOSTS.some(
      (supported) => host === supported || host.endsWith(`.${supported}`),
    );
  } catch {
    return false;
  }
}
