import { commands } from "@/lib/vecdir/bindings";

export async function processSpace(spaceId: number): Promise<boolean> {
    const result = await commands.processSpace(spaceId);

    console.log(result);

    if (result.status === "ok") {
        return true;
    }

    return false;
}
