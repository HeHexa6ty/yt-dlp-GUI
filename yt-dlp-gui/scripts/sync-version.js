
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

const packageJson = JSON.parse(
  fs.readFileSync(path.join(root, "package.json"), "utf8")
);

const version = packageJson.version;

// Update Tauri configuration
const tauriPath = path.join(root, "src-tauri/tauri.conf.json");
const tauriConfig = JSON.parse(fs.readFileSync(tauriPath, "utf8"));

tauriConfig.version = version;

fs.writeFileSync(
  tauriPath,
  JSON.stringify(tauriConfig, null, 2) + "\n"
);

// Update Cargo.toml
const cargoPath = path.join(root, "src-tauri/Cargo.toml");
let cargoToml = fs.readFileSync(cargoPath, "utf8");

const updatedCargo = cargoToml.replace(
  /^(\[package\][\s\S]*?^version\s*=\s*)"[^"]+"/m,
  `$1"${version}"`
);

if (updatedCargo === cargoToml && !cargoToml.includes(`version = "${version}"`)) {
  throw new Error("Could not update Cargo.toml version.");
}

fs.writeFileSync(cargoPath, updatedCargo);

console.log(`Synchronized project version to ${version}`);
