import { createRoute } from "@tanstack/react-router";
import { rootRoute } from "@/routes/root";

function Index() {
    return (
        <div className="p-2">
            <h3>Welcome Home!</h3>
        </div>
    );
}

export const indexRoute = createRoute({
    getParentRoute: () => rootRoute,
    path: "/",
    component: Index,
});
