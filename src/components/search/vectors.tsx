import type { VectorSearchResult } from "@/lib/vecdir/bindings";
import { convertFileSrc } from "@tauri-apps/api/core";
import { Database, FileCode } from "lucide-react";
import { Badge } from "@/components/ui/badge";
import { Card } from "@/components/ui/card";

function getImage(path: string) {
    const res = convertFileSrc(path);

    return res;
}

export function VectorsSearchColumn({ results }: { results: VectorSearchResult[] }) {
    results.forEach((result) => {
        getImage(result.absolute_path);
    });
    return (
        <div className="flex flex-col min-h-0">
            <div className="flex items-center gap-2 mb-4 shrink-0">
                <Database className="h-5 w-5 text-primary" />
                <h3 className="text-lg font-semibold">Vector Search</h3>
                <Badge variant="secondary" className="ml-auto">
                    {results.length}
                </Badge>
            </div>
            <Card className="flex-1 min-h-0 bg-card border-border overflow-hidden flex flex-col">
                <div className="p-4 overflow-y-auto flex-1">
                    <div className="space-y-3">
                        {results.map(result => (
                            <div
                                key={result.file_id}
                                className="p-4 rounded-md bg-secondary/50 hover:bg-secondary transition-colors cursor-pointer"
                            >
                                <div className="flex items-center justify-between mb-3">
                                    <div className="font-medium text-foreground flex items-center gap-2">
                                        <FileCode className="h-4 w-4 text-muted-foreground shrink-0" />
                                        <span className="font-mono text-sm">{result.filename}</span>
                                    </div>
                                    <Badge variant="outline" className="text-primary border-primary shrink-0">
                                        {(100 - result.distance * 100).toFixed(0)}
                                        %
                                    </Badge>
                                </div>
                                <img src={getImage(result.absolute_path)} />

                                {/* <pre className="text-xs font-mono text-muted-foreground bg-background p-2 rounded overflow-x-auto leading-relaxed">
                                </pre> */}
                            </div>
                        ))}
                    </div>
                </div>
            </Card>
        </div>
    );
}
