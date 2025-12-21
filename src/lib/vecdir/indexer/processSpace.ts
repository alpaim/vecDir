import { commands } from "@/lib/vecdir/bindings";

export async function processSpace(spaceId: number): Promise<boolean> {
    const result = await commands.processSpace(spaceId);

    if (result.status === "ok") {
        return true;
    }

    return false;
}
