import { commands } from "@/lib/vecdir/bindings";

export async function addRoot(spaceId: number, path: string): Promise<number | null> {
    const result = await commands.addRoot(spaceId, path);

    if (result.status === "ok") {
        return result.data;
    }

    return result.error;
}
