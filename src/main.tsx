import { createRouter, RouterProvider } from "@tanstack/react-router";
import { StrictMode } from "react";
import ReactDOM from "react-dom/client";
import { createSpaceRoute } from "@/routes/createSpace";
import { indexRoute } from "@/routes/index";
import { rootRoute } from "@/routes/root";
import { settingsRoute } from "@/routes/settings";

import "@/index.css";

const routeTree = rootRoute.addChildren([indexRoute, settingsRoute, createSpaceRoute]);

const router = createRouter({ routeTree });

declare module "@tanstack/react-router" {
    interface Register {
        router: typeof router;
    }
}

const rootElement = document.getElementById("root")!;
if (!rootElement.innerHTML) {
    const root = ReactDOM.createRoot(rootElement);
    root.render(
        <StrictMode>
            <RouterProvider router={router} />
        </StrictMode>,
    );
}
