import { createRootRoute, Outlet } from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/react-router-devtools";
import { Header } from "@/components/header/header";
import { Statusbar } from "@/components/statusbar/statusbar";
import { useBackendReady } from "@/hooks/useBackendReady";

function RootLayout() {
    useBackendReady();

    return (
        <div className="flex flex-col h-screen">
            <Header />

            <main className="flex-1 overflow-auto">
                <Outlet />
            </main>

            <Statusbar />

            <TanStackRouterDevtools position="top-right" />
        </div>
    );
}

export const rootRoute = createRootRoute({ component: RootLayout });
