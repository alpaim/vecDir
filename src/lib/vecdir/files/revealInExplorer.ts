import { commands } from "@/lib/vecdir/bindings";

export async function revealInExplorer(path: string): Promise<void> {
    await commands.revealInExplorer(path);
}
