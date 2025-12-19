import { Link } from "@tanstack/react-router";
import { RefreshCcw, Settings } from "lucide-react";
import { Logo } from "@/components/index/logo";
import { Search } from "@/components/search/search";
import { Button } from "@/components/ui/button";

export function Main({ searchQuery }: { searchQuery: string | undefined }) {
    if (searchQuery !== undefined && searchQuery !== "") {
        return (
            <Search searchQuery={searchQuery} />
        );
    }

    return (
        <div className="flex flex-col items-center justify-center flex-wrap w-full h-full">
            <div className="shrink-0">
                <Logo />
            </div>
            <div className="w-full justify-center shrink-0">
                <div className="flex flex-row gap-3 justify-center pt-6">
                    <Link to="/settings">
                        <Button>
                            <RefreshCcw />
                        </Button>
                    </Link>
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
