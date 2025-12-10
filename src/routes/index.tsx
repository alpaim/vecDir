import { createRoute } from "@tanstack/react-router";
import { useState } from "react";
import { Input } from "@/components/ui/input";
import { rootRoute } from "@/routes/root";

function SearchResult({ searchQuery}: { searchQuery: string }) {
    return (
        <div>
            Your query:
            {" "}
            {searchQuery}
        </div>
    );
}

function Logo() {
    return (
        <div>
            <h3 className="font-mono text-2xl font-extrabold">VecDir</h3>
        </div>
    );
}

function Index() {
    const [searchQeury, setSearchQuery] = useState<string | undefined>();

    return (
        <div className="relative flex items-center justify-center h-full w-full">
            <div className="absolute top-5 left-0 w-full px-5">
                <Input placeholder="Search..." className="font-mono" onChange={e => setSearchQuery(e.target.value)} value={searchQeury} />
            </div>
            <div className="flex">
                {
                    (searchQeury !== undefined && searchQeury !== "") ? <SearchResult searchQuery={searchQeury} /> : <Logo />
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
