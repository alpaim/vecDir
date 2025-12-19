import { FileCode, FileText } from "lucide-react";
import { Badge } from "@/components/ui/badge";
import { Card } from "@/components/ui/card";

interface DescritionsResultProps {
    results: {
        id: number;
        file: string;
        description: string;
    }[];
}

export function DescriptionsColumn({ results }: DescritionsResultProps) {
    return (
        <div className="flex flex-col min-h-0">
            <div className="flex items-center gap-2 mb-4 shrink-0">
                <FileText className="h-5 w-5 text-primary" />
                <h3 className="text-lg font-semibold">Descriptions</h3>
                <Badge variant="secondary" className="ml-auto">
                    {results.length}
                </Badge>
            </div>
            <Card className="flex-1 min-h-0 bg-card border-border overflow-hidden flex flex-col">
                <div className="p-4 overflow-y-auto flex-1">
                    <div className="space-y-3">
                        {results.map(result => (
                            <div
                                key={result.id}
                                className="p-4 rounded-md bg-secondary/50 hover:bg-secondary transition-colors cursor-pointer"
                            >
                                <div className="font-medium text-foreground mb-2 flex items-center gap-2">
                                    <FileCode className="h-4 w-4 text-muted-foreground shrink-0" />
                                    <span className="font-mono text-sm">{result.file}</span>
                                </div>
                                <p className="text-sm text-muted-foreground leading-relaxed">{result.description}</p>
                            </div>
                        ))}
                    </div>
                </div>
            </Card>
        </div>
    );
}
