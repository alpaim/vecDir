import { createRoute } from "@tanstack/react-router";
import { useState } from "react";
import { Initializing } from "@/components/index/initializing";
import { Main } from "@/components/index/main";
import { Input } from "@/components/ui/input";
import { rootRoute } from "@/routes/root";
import { useAppState } from "@/store/store";

function Index() {
    const store = useAppState();
    const [searchQeury, setSearchQuery] = useState<string | undefined>();

    return (
        <div className="flex flex-col h-full w-full">
            <div className="w-full top-5 left-0 px-5 pt-5">
                <Input placeholder="Search..." className="font-mono" onChange={e => setSearchQuery(e.target.value)} value={searchQeury} />
            </div>
            <div className="flex-1 w-full h-full">
                {
                    store.isBackendReady ? <Main searchQuery={searchQeury} /> : <Initializing />
                }
            </div>
        </div>
    );
}

export const indexRoute = createRoute({
    getParentRoute: () => rootRoute,
    path: "/",
    component: Index,
});
