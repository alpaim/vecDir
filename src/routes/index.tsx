import { createRoute } from "@tanstack/react-router";
import { Input } from "@/components/ui/input";
import { rootRoute } from "@/routes/root";

function Index() {
    return (
        <div className="relative flex items-center justify-center h-full w-full">
            <div className="absolute top-5 left-0 w-full px-5">
                <Input placeholder="Search..." className="font-mono" />
            </div>
            <div className="flex">
                <div>
                    <h3 className="font-mono text-2xl font-extrabold">VecDir</h3>
                </div>
            </div>
        </div>
    );
}

export const indexRoute = createRoute({
    getParentRoute: () => rootRoute,
    path: "/",
    component: Index,
});
