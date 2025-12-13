import type { EmbeddingConfig, LLMConfig } from "@/lib/vecdir/bindings";
import { commands } from "@/lib/vecdir/bindings";

export async function createSpace(name: string, llmConfig: LLMConfig, embeddingConfig: EmbeddingConfig): Promise<boolean> {
    const result = await commands.createSpace(name, llmConfig, embeddingConfig);

    if (result.status === "ok") {
        return true;
    }

    return false;
}
