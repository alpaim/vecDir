import { writeImage } from "@tauri-apps/plugin-clipboard-manager";

export async function copyImageToClipboard(path: string): Promise<void> {
    await writeImage(path);
}
