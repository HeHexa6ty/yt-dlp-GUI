// Small button that pastes the clipboard into the URL field and analyzes it
import "./PasteButton.css";

interface PasteButtonProps {
  onClick: () => void;
  disabled?: boolean;
}

export default function PasteButton({
  onClick,
  disabled = false,
}: PasteButtonProps) {
  return (
    <button
      type="button"
      className="paste-button"
      onClick={onClick}
      disabled={disabled}
      aria-label="Paste link from clipboard"
      title="Paste link from clipboard"
    >
      <svg
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        strokeWidth="2"
        strokeLinecap="round"
        strokeLinejoin="round"
        aria-hidden="true"
      >
        <rect x="8" y="2" width="8" height="4" rx="1" />
        <path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2" />
      </svg>
    </button>
  );
}
