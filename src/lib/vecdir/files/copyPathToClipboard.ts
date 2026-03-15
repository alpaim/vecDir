import { writeText } from "@tauri-apps/plugin-clipboard-manager";

export async function copyPathToClipboard(path: string): Promise<void> {
    await writeText(path);
}
