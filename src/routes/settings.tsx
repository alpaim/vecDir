import { createRoute } from "@tanstack/react-router";
import { rootRoute } from "@/routes/root";

function Settings() {
    return <div className="p-2">Hello from Settings!</div>;
}

export const settingsRoute = createRoute({
    getParentRoute: () => rootRoute,
    path: "/settings",
    component: Settings,
});
