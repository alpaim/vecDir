import { useEffect } from "react";
import { Button } from "@/components/ui/button";
import { createSpace } from "@/lib/vecdir/spaces/createSpace";
import { getAllSpaces } from "@/lib/vecdir/spaces/getSpace";
import { useAppState } from "@/store/store";

export function CreateSpace() {
    const store = useAppState();

    // revalidating if there are no spaces on backend
    useEffect(() => {
        getAllSpaces().then((spaces) => {
            if (spaces.length > 0) {
                store.setSpaces(spaces);
            }
        });
    }, []);

    return (
        <div>
            <Button onClick={async () => {
                const space = await createSpace("default", {
                    model: "mistralai/ministral-3-3b",
                    system_prompt: "Describe the following image",
                    user_prompt: "Describe this image",
                }, {
                    model: "text-embedding-qwen3-embedding-0.6b",
                    dimensions: 1024,
                });

                if (space !== undefined) {
                    store.addSpace(space);
                }
            }}
            >
                Create Default Space
            </Button>
        </div>
    );
}
