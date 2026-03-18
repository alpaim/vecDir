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
        <div className="grid grid-cols-3 sm:grid-cols-4 md:grid-cols-5 lg:grid-cols-6 xl:grid-cols-8 gap-3 p-4">{results.map(result => (
                <ContextMenu key={result.file_id}>
                    <ContextMenuTrigger asChild>
                        <div
                            className="aspect-square rounded-xl bg-secondary/50 hover:bg-secondary transition-colors cursor-pointer border border-transparent hover:border-border/50 group overflow-hidden relative"
                            onClick={() => handleOpen(result.absolute_path)}
                        >
                            {isImageFile(result.absolute_path) && (
                                <img
                                    src={getImage(result.absolute_path)}
                                    alt={result.filename}
                                    className="w-full h-full object-cover block group-hover:scale-105 transition-transform duration-300"
                                    loading="lazy"
                                />
                            )}

                            <div className="absolute inset-0 bg-gradient-to-t from-black/60 to-transparent opacity-0 group-hover:opacity-100 transition-opacity flex flex-col justify-end p-2">
                                <span className="font-mono text-[10px] text-white truncate" title={result.filename}>
                                    {result.filename}
                                </span>
                                <Badge variant="outline" className="text-[9px] px-1 h-4 text-white border-white/30 w-fit mt-1">
                                    {(100 - result.distance * 100).toFixed(0)}
                                    %
                                </Badge>
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
