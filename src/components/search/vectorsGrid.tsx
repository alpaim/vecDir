import type { VectorSearchResult } from "@/lib/vecdir/bindings";
import { convertFileSrc } from "@tauri-apps/api/core";
import { Badge } from "@/components/ui/badge";

function getImage(path: string) {
    const res = convertFileSrc(path);

    return res;
}

export function VectorsSearchGrid({ results }: { results: VectorSearchResult[] }) {
    if (!results || results.length === 0) {
        return <div className="text-center p-10 text-muted-foreground">No Results</div>;
    }

    return (
        <div className="columns-1 sm:columns-2 md:columns-3 lg:columns-4 xl:columns-5 gap-4 px-4 pb-4">
            {results.map(result => (
                <div
                    key={result.file_id}
                    className="break-inside-avoid mb-4 p-3 rounded-xl bg-secondary/50 hover:bg-secondary transition-colors cursor-pointer border border-transparent hover:border-border/50 group"
                >
                    <div className="flex items-center justify-between mb-2">
                        <div className="font-medium text-foreground flex items-center gap-2 overflow-hidden min-w-0">
                            <span className="font-mono text-xs truncate opacity-70 group-hover:opacity-100 transition-opacity" title={result.filename}>
                                {result.filename}
                            </span>
                        </div>
                        <Badge variant="outline" className="text-[10px] px-1.5 h-5 text-primary border-primary/30 shrink-0">
                            {(100 - result.distance * 100).toFixed(0)}
                            %
                        </Badge>
                    </div>

                    <div className="rounded-lg overflow-hidden bg-background/50">
                        <img
                            src={getImage(result.absolute_path)}
                            alt={result.filename}
                            className="w-full h-auto object-cover block hover:scale-105 transition-transform duration-300"
                            loading="lazy"
                        />
                    </div>
                </div>
            ))}
        </div>
    );
}
