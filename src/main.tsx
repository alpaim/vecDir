import { createRouter, RouterProvider } from "@tanstack/react-router";
import { StrictMode } from "react";
import ReactDOM from "react-dom/client";
import { indexRoute } from "@/routes";
import { rootRoute } from "@/routes/__root";
import { aboutRoute } from "@/routes/about";

import "@/index.css";

const routeTree = rootRoute.addChildren([indexRoute, aboutRoute]);

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
