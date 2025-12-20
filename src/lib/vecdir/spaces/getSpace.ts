import type { Space } from "@/lib/vecdir/bindings";
import { commands } from "@/lib/vecdir/bindings";

export async function getAllSpaces(): Promise<Space[]> {
    const result = await commands.getAllSpaces();

    if (result.status === "ok") {
        return result.data;
    }

    return [];
}

export async function getSpaceById(spaceId: number): Promise<Space | undefined> {
    const result = await commands.getSpaceById(spaceId);

    if (result.status === "ok") {
        return result.data;
    }

    return undefined;
}
