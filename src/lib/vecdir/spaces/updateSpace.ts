import type { EmbeddingConfig, LLMConfig } from "@/lib/vecdir/bindings";
import { commands } from "@/lib/vecdir/bindings";

export async function updateSpace(spaceId: number, name: string, description: string, llmConfig: LLMConfig, embeddingConfig: EmbeddingConfig): Promise<boolean> {
    const result = await commands.updateSpace(spaceId, name, description, llmConfig, embeddingConfig);

    if (result.status === "error") {
        return false;
    }

    return result.data;
}
