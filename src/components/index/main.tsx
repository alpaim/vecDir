import { Logo } from "@/components/index/logo";
import { SpaceSettings } from "@/components/index/spaceSettings";
import { Search } from "@/components/search/search";

export function Main({ searchQuery }: { searchQuery: string | undefined }) {
    if (searchQuery !== undefined && searchQuery !== "") {
        return (
            <Search searchQuery={searchQuery} />
        );
    }

    return (
        <div className="flex flex-wrap w-full">
            <Logo />
            <SpaceSettings />
        </div>
    );
}
