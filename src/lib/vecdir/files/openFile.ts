import { openPath } from "@tauri-apps/plugin-opener";

export async function openFile(path: string): Promise<void> {
    await openPath(path);
}
