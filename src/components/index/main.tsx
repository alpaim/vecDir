import type { Space } from "@/lib/vecdir/bindings";
import { Link, useNavigate } from "@tanstack/react-router";
import { RefreshCcw, Settings } from "lucide-react";
import { useEffect, useState } from "react";
import { Logo } from "@/components/index/logo";
import { Search } from "@/components/search/search";
import { Button } from "@/components/ui/button";
import { processSpace } from "@/lib/vecdir/indexer/processSpace";
import { indexSpace } from "@/lib/vecdir/indexer/startIndexing";
import { getSpaceById } from "@/lib/vecdir/spaces/getSpace";
import { useAppState } from "@/store/store";

async function onIndexSpace(spaceId: number) {
    await indexSpace(spaceId);
    await processSpace(spaceId);
}

export function Main({ searchQuery }: { searchQuery: string | undefined }) {
    const [space, setSpace] = useState<Space | undefined>();

    const spaces = useAppState(state => state.spaces);
    const selectedSpace = useAppState(state => state.selectedSpace);

    const navigate = useNavigate({ from: "/" });

    useEffect(() => {
        getSpaceById(selectedSpace).then((s) => {
            if (s === undefined) {
                navigate({ to: "/createSpace" });
                return;
            }

            setSpace(s);
        });
    }, [selectedSpace]);

    useEffect(() => {
        if (spaces.size <= 0) {
            navigate({ to: "/createSpace" });
        }
    }, [spaces]);

    if (searchQuery !== undefined && searchQuery !== "") {
        return (
            <Search searchQuery={searchQuery} />
        );
    }

    return (
        <div className="flex flex-col items-center justify-center flex-wrap w-full h-full">
            <div className="shrink-0">
                <Logo />

                <div className="text-center">
                    Selected Space:
                    {" "}
                    {space?.name}
                </div>
            </div>
            <div className="w-full justify-center shrink-0">
                <div className="flex flex-row gap-3 justify-center pt-6">
                    <Button onClick={async () => onIndexSpace(selectedSpace).then(() => {})}>
                        <RefreshCcw />
                    </Button>
                    <Link to="/settings">
                        <Button>
                            <Settings />
                        </Button>
                    </Link>
                </div>
            </div>
        </div>
    );
}
