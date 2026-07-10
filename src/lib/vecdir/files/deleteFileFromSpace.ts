import { commands } from "@/lib/vecdir/bindings";

export async function deleteFileFromSpace(spaceId: number, fileId: number): Promise<boolean> {
    const result = await commands.deleteFileFromSpace(spaceId, fileId);

    return result.status === "ok" && result.data;
}
