import type { VectorSearchResult } from "@/lib/vecdir/bindings";
import { convertFileSrc } from "@tauri-apps/api/core";
import { Copy, FolderOpen, Info, RefreshCw, Trash2 } from "lucide-react";
import { Badge } from "@/components/ui/badge";
import {
    ContextMenu,
    ContextMenuContent,
    ContextMenuItem,
    ContextMenuSeparator,
    ContextMenuTrigger,
} from "@/components/ui/context-menu";
import { copyImageToClipboard } from "@/lib/vecdir/files/copyImageToClipboard";
import { copyPathToClipboard } from "@/lib/vecdir/files/copyPathToClipboard";
import { isImageFile } from "@/lib/vecdir/files/isImageFile";
import { openFile } from "@/lib/vecdir/files/openFile";
import { revealInExplorer } from "@/lib/vecdir/files/revealInExplorer";

function getImage(path: string) {
    const res = convertFileSrc(path);

    return res;
}

function handleOpen(path: string) {
    openFile(path);
}

function handleReveal(path: string) {
    revealInExplorer(path);
}

function handleCopyPath(path: string) {
    copyPathToClipboard(path);
}

function handleCopyImage(path: string) {
    copyImageToClipboard(path);
}

export function VectorsSearchGrid({ results }: { results: VectorSearchResult[] }) {
    if (!results || results.length === 0) {
        return <div className="text-center p-10 text-muted-foreground">No Results</div>;
    }

    return (
        <div className="columns-1 sm:columns-2 md:columns-3 lg:columns-4 xl:columns-5 gap-4 px-4 pb-4">
            {results.map(result => (
                <ContextMenu key={result.file_id}>
                    <ContextMenuTrigger asChild>
                        <div
                            className="break-inside-avoid mb-4 p-3 rounded-xl bg-secondary/50 hover:bg-secondary transition-colors cursor-pointer border border-transparent hover:border-border/50 group"
                            onClick={() => handleOpen(result.absolute_path)}
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
                    </ContextMenuTrigger>
                    <ContextMenuContent>
                        <ContextMenuItem onClick={() => handleOpen(result.absolute_path)}>
                            Open
                        </ContextMenuItem>
                        <ContextMenuItem onClick={() => handleReveal(result.absolute_path)}>
                            <FolderOpen className="mr-2 h-4 w-4" />
                            Open in Directory
                        </ContextMenuItem>
                        {isImageFile(result.absolute_path)
                            ? (
                                    <ContextMenuItem onClick={() => handleCopyImage(result.absolute_path)}>
                                        <Copy className="mr-2 h-4 w-4" />
                                        Copy
                                    </ContextMenuItem>
                                )
                            : (
                                    <ContextMenuItem disabled>
                                        <Copy className="mr-2 h-4 w-4" />
                                        Copy
                                    </ContextMenuItem>
                                )}
                        <ContextMenuItem onClick={() => handleCopyPath(result.absolute_path)}>
                            <Copy className="mr-2 h-4 w-4" />
                            Copy as Path
                        </ContextMenuItem>
                        <ContextMenuSeparator />
                        <ContextMenuItem>
                            <Info className="mr-2 h-4 w-4" />
                            Info
                        </ContextMenuItem>
                        <ContextMenuItem>
                            <RefreshCw className="mr-2 h-4 w-4" />
                            Rescan
                        </ContextMenuItem>
                        <ContextMenuSeparator />
                        <ContextMenuItem variant="destructive">
                            <Trash2 className="mr-2 h-4 w-4" />
                            Delete from Space
                        </ContextMenuItem>
                    </ContextMenuContent>
                </ContextMenu>
            ))}
        </div>
    );
}
