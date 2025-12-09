import { createRoute } from "@tanstack/react-router";
import { rootRoute } from "@/routes/root";

function About() {
    return <div className="p-2">Hello from About!</div>;
}

export const aboutRoute = createRoute({
    getParentRoute: () => rootRoute,
    path: "/about",
    component: About,
});
