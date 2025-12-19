import type { VectorSearchResult } from "@/lib/vecdir/bindings";
import { useEffect, useState } from "react";
import { searchVectors } from "@/lib/vecdir/search/searchVectors";
import { useAppState } from "@/store/store";
import { DescriptionsColumn } from "./descriptions";
import { PathnamesColumn } from "./pathnames";
import { VectorsSearchColumn } from "./vectors";

export interface SearchParams {
    searchQuery: string;
}

const mockPathnameResults = [
    { id: 1, path: "/users/documents/projects/react-app/src/components/Button.tsx", type: "file" },
    { id: 2, path: "/users/documents/projects/react-app/src/utils/helpers.ts", type: "file" },
    { id: 3, path: "/users/documents/reports/2024/quarterly-report.pdf", type: "file" },
    { id: 4, path: "/users/documents/designs/figma-exports/", type: "directory" },
    { id: 5, path: "/users/documents/projects/api/controllers/auth.js", type: "file" },
];

const mockDescriptionResults = [
    {
        id: 1,
        file: "authentication.ts",
        description: "User authentication service with JWT token management and session handling",
    },
    {
        id: 2,
        file: "database-config.ts",
        description: "Database configuration and connection pooling setup for PostgreSQL",
    },
    { id: 3, file: "api-routes.ts", description: "REST API route definitions with Express middleware" },
    {
        id: 4,
        file: "validation.ts",
        description: "Input validation utilities using Zod schema validation",
    },
];

export function Search({ searchQuery }: SearchParams) {
    const [pathnamesResult, setPathnamesResult] = useState<typeof mockPathnameResults>([]);
    const [descriptionsResult, setDescriptionsResult] = useState<typeof mockDescriptionResults>([]);
    const [vectorsResult, setVectorsResult] = useState<VectorSearchResult[]>([]);

    const spaceId = useAppState(state => state.selectedSpace);

    useEffect(() => {
        searchVectors(spaceId, searchQuery, 10).then((result) => {
            setVectorsResult(result);
        });
    }, [searchQuery]);

    return (
        <div className="grid grid-cols-3 gap-6 flex-1 h-full min-h-0 w-full p-5">
            {/* Pathnames Column */}
            <PathnamesColumn results={mockPathnameResults} />

            {/* Descriptions Column */}
            <DescriptionsColumn results={mockDescriptionResults} />

            {/* Vector Results Column */}
            <VectorsSearchColumn results={vectorsResult} />
        </div>
    );
}
