import { useEffect } from "react";
import { commands, events } from "@/lib/vecdir/bindings";
import { useAppState } from "@/store/store";

export function useBackendReady() {
    const setBackendStatus = useAppState(state => state.setBackendStatus);

    useEffect(() => {
        let isMounted = true;

        commands.checkIsStateReady().then((state) => {
            console.log("Backend State status received:", state);

            if (isMounted && state) {
                setBackendStatus(state);
            }
        });

        return () => {
            isMounted = false;
        };
    }, []);

    useEffect(() => {
        let isMounted = true;

        const unlistedPromise = events.backendReadyEvent.listen((event) => {
            console.log("Event received:", event);

            if (isMounted) {
                setBackendStatus(true);
            }
        });

        return () => {
            isMounted = false;

            unlistedPromise.then(unlisten => unlisten());
        };
    }, []);
}
