import { createRoute } from "@tanstack/react-router";
import { Settings as SettingsComponent } from "@/components/settings/settings";
import { rootRoute } from "@/routes/root";

function Settings() {
    return <SettingsComponent />;
}

export const settingsRoute = createRoute({
    getParentRoute: () => rootRoute,
    path: "/settings",
    component: Settings,
});
