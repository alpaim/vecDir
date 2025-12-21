import type { StatusEvent } from "@/lib/vecdir/bindings";
import { useEffect, useState } from "react";
import { toast } from "sonner";
import { events } from "@/lib/vecdir/bindings";

export function Statusbar() {
    const [indexing, setIndexing] = useState<StatusEvent>();
    const [processing, setProcessing] = useState<StatusEvent>();
    const [idle, setIdle] = useState<StatusEvent | boolean>(true);

    useEffect(() => {
        const unlistedPromise = events.statusEvent.listen((event) => {
            if (event.payload.status === "Indexing") {
                setIndexing(event.payload);

                setIdle(false);
            }
            else if (event.payload.status === "Processing") {
                setProcessing(event.payload);

                setIdle(false);
            }
            else if (event.payload.status === "Idle") {
                setIndexing(undefined);
                setProcessing(undefined);

                setIdle(event.payload);
            }
            else if (event.payload.status === "Notification") {
                toast(event.payload.message);
            }
            else if (event.payload.status === "Error") {
                toast.error(event.payload.message);
            }
        });

        return () => {
            unlistedPromise.then(unlisten => unlisten());
        };
    }, []);

    return (
        <footer className="flex flex-row-reverse content-center px-4 border-t h-8 bg-blue-500 text-white font-mono">
            {
                indexing
                    ? (
                            <div>
                                Indexing Space
                            </div>
                        )
                    : null
            }
            {
                processing
                    ? (
                            <div>
                                Processing Space:
                                {" "}
                                {processing.processed ? processing.processed : 0}
                                {" "}
                                of
                                {" "}
                                {processing.total ? processing.total : 0}
                            </div>
                        )
                    : null
            }
            {
                idle
                    ? (
                            <div>
                                Idle
                            </div>
                        )
                    : null

            }
        </footer>
    );
}
