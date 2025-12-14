import type { IndexedRoot } from "@/lib/vecdir/bindings";
import { X } from "lucide-react";
import { Button } from "@/components/ui/button";

export function RootsList({ roots}: { roots: IndexedRoot[] }) {
    return (
        <ul className="space-y-2">
            {
                roots.map(root => (
                    <li
                        key={root.id}
                        className="flex items-center justify-between p-2 rounded-md border bg-card hover:bg-accent/50 transition-colors"
                    >
                        <span>{root.path}</span>
                        <Button
                            variant="ghost"
                            size="icon"
                            className="h-8 w-8 text-muted-foreground hover:text-destructive"
                            onClick={() => {}}
                        >
                            <X className="h-4 w-4" />
                        </Button>
                    </li>
                ))
            }
        </ul>
    );
}
