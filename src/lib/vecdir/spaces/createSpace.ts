import type { EmbeddingConfig, LLMConfig, Space } from "@/lib/vecdir/bindings";
import { commands } from "@/lib/vecdir/bindings";
import { getSpaceById } from "@/lib/vecdir/spaces/getSpace";

export async function createSpace(name: string, description: string, llmConfig: LLMConfig, embeddingConfig: EmbeddingConfig): Promise<Space | undefined> {
    const spaceId = await commands.createSpace(name, description, llmConfig, embeddingConfig);

    if (spaceId.status === "error") {
        return undefined;
    }

    if (spaceId.data === null) {
        return undefined;
    }

    const createdSpace = await getSpaceById(spaceId.data);

    if (!createdSpace) {
        return undefined;
    }

    return createdSpace;
}
