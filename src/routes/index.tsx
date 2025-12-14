import { createRoute } from "@tanstack/react-router";
import { useState } from "react";
import { Search } from "@/components/search/search";
import { Input } from "@/components/ui/input";
import { rootRoute } from "@/routes/root";
import { useAppState } from "@/store/store";

function Logo() {
    return (
        <div>
            <h3 className="font-mono text-2xl font-extrabold">VecDir</h3>
        </div>
    );
}

function Initializing() {
    return (
        <div>
            <h3 className="font-mono text-2xl font-extrabold">Initialzing</h3>
        </div>
    );
}

function Index() {
    const store = useAppState();
    const [searchQeury, setSearchQuery] = useState<string | undefined>();

    return (
        <div className="relative flex items-center justify-center h-full w-full">
            <div className="absolute top-5 left-0 w-full px-5">
                <Input placeholder="Search..." className="font-mono" onChange={e => setSearchQuery(e.target.value)} value={searchQeury} />
            </div>
            <div className="flex">
                {

                }
                {
                    store.isBackendReady
                        ? (
                                (searchQeury !== undefined && searchQeury !== "") ? <Search searchQuery={searchQeury} /> : <Logo />
                            )
                        : <Initializing />
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
