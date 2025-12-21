import type { IndexedRoot } from "@/lib/vecdir/bindings";
import { commands } from "@/lib/vecdir/bindings";

export async function getRootsBySpaceId(spaceId: number): Promise<IndexedRoot[]> {
    const result = await commands.getRootsBySpaceId(spaceId);

    if (result.status === "ok") {
        return result.data;
    }

    return [];
}
