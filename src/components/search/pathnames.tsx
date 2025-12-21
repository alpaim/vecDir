import { FileCode, FileText, Folder } from "lucide-react";
import { Badge } from "@/components/ui/badge";
import { Card } from "@/components/ui/card";

interface PathnamesResultProps {
    results: {
        id: number;
        path: string;
        type: string;
    }[];
}

export function PathnamesColumn({ results }: PathnamesResultProps) {
    return (
        <div className="flex flex-col min-h-0">
            <div className="flex items-center gap-2 mb-4 shrink-0">
                <FileText className="h-5 w-5 text-primary" />
                <h3 className="text-lg font-semibold">Pathnames</h3>
                <Badge variant="secondary" className="ml-auto">
                    {results.length}
                </Badge>
            </div>
            <Card className="flex-1 min-h-0 bg-card border-border overflow-hidden flex flex-col">
                <div className="p-4 overflow-y-auto flex-1">
                    <div className="space-y-2">
                        {results.map(result => (
                            <div
                                key={result.id}
                                className="p-3 rounded-md bg-secondary/50 hover:bg-secondary transition-colors cursor-pointer"
                            >
                                <div className="flex items-start gap-2">
                                    {result.type === "file"
                                        ? (
                                                <FileCode className="h-4 w-4 text-muted-foreground shrink-0 mt-0.5" />
                                            )
                                        : (
                                                <Folder className="h-4 w-4 text-muted-foreground shrink-0 mt-0.5" />
                                            )}
                                    <span className="text-sm font-mono text-foreground break-all leading-relaxed">{result.path}</span>
                                </div>
                            </div>
                        ))}
                    </div>
                </div>
            </Card>
        </div>
    );
}
