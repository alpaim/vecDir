import { commands } from "@/lib/vecdir/bindings";

export async function indexSpace(spaceId: number): Promise<boolean> {
    const result = await commands.indexSpace(spaceId);

    if (result.status === "ok") {
        return result.data;
    }

    return false;
}
