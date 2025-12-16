import type { IndexedRoot } from "@/lib/vecdir/bindings";
import { open } from "@tauri-apps/plugin-dialog";
import { useEffect, useState } from "react";
import { RootsList } from "@/components/index/rootsList";
import { CreateSpace } from "@/components/spaces/createSpace";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardFooter, CardHeader, CardTitle } from "@/components/ui/card";
import { processSpace } from "@/lib/vecdir/indexer/processSpace";
import { indexSpace } from "@/lib/vecdir/indexer/startIndexing";
import { addRoot } from "@/lib/vecdir/roots/createRoot";
import { getRootsBySpaceId } from "@/lib/vecdir/roots/getRoot";
import { useAppState } from "@/store/store";

export function SpaceSettings() {
    const [roots, setRoots] = useState<IndexedRoot[]>([]);

    const store = useAppState();

    async function updateRoots(selectedSpaceId: number, set: (r: IndexedRoot[]) => void): Promise<void> {
        const newRoots = await getRootsBySpaceId(selectedSpaceId);

        set(newRoots);
    }

    useEffect(() => {
        updateRoots(store.selectedSpace, setRoots).then(() => {});
    }, []);

    if (store.spaces.size === 0) {
        return <CreateSpace />;
    }

    return (
        <div className="flex justify-around w-full">
            <Card className="w-full max-w-3xs">
                <CardHeader>
                    <CardTitle>
                        Directories
                    </CardTitle>
                </CardHeader>
                <CardContent>
                    <RootsList roots={roots} />
                </CardContent>
                <CardFooter className="flex-col gap-2">
                    <Button
                        variant="outline"
                        className="w-full"
                        onClick={async () => {
                            const path = await open({
                                multiple: false,
                                directory: true,
                            });

                            if (!path) {
                                return;
                            }

                            await addRoot(store.selectedSpace, path);

                            await updateRoots(store.selectedSpace, setRoots);

                            console.log(path, store.spaces.get(1), store.selectedSpace);
                        }}
                    >
                        Add Directory
                    </Button>
                </CardFooter>
            </Card>
            <Card className="w-full max-w-3xs">
                <CardHeader>
                    <CardTitle>
                        AI Settings
                    </CardTitle>
                </CardHeader>
                <CardContent>
                    asdas
                </CardContent>
            </Card>
            <div className="flex flex-1">
                <Button onClick={() => {
                    indexSpace(store.selectedSpace).then(() => {});
                }}
                >
                    INDEX SPACE
                </Button>
                <Button onClick={() => {
                    processSpace(store.selectedSpace).then(() => {});
                }}
                >
                    PROCESS SPACE
                </Button>
            </div>

        </div>
    );
}
