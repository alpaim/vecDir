import { createRoute } from "@tanstack/react-router";
import { CreateSpace as CreateSpaceComponent } from "@/components/settings/createSpace";
import { rootRoute } from "@/routes/root";

function CreateSpace() {
    return <CreateSpaceComponent />;
}

export const createSpaceRoute = createRoute({
    getParentRoute: () => rootRoute,
    path: "/createSpace",
    component: CreateSpace,
});
