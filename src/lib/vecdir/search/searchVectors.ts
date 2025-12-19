import type { VectorSearchResult } from "../bindings";
import { commands } from "@/lib/vecdir/bindings";

export async function searchVectors(spaceId: number, query: string, limit: number): Promise<VectorSearchResult[]> {
    const result = await commands.searchByEmdedding(spaceId, query, limit);

    if (result.status === "error") {
        return [];
    }

    return result.data;
}
