import type { FileMetadata } from "@/lib/vecdir/bindings";
import { commands } from "@/lib/vecdir/bindings";

export async function getFilesById(ids: number[]): Promise<FileMetadata[]> {
    const result = await commands.getFilesByIds(ids);

    if (result.status === "ok") {
        return result.data;
    }

    return [];
}
