import { commands } from "@/lib/vecdir/bindings";

export async function deleteRoot(rootId: number): Promise<boolean> {
    const result = await commands.deleteRoot(rootId);

    if (result.status === "ok") {
        return true;
    }

    return false;
}
