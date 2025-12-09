import { createRoute } from "@tanstack/react-router";
import { rootRoute } from "@/routes/__root";

function About() {
    return <div className="p-2">Hello from About!</div>;
}

export const aboutRoute = createRoute({
    getParentRoute: () => rootRoute,
    path: "/about",
    component: About,
});
