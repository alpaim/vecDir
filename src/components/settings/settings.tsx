import type { EmbeddingConfig, IndexedRoot, LLMConfig } from "@/lib/vecdir/bindings";
import { useForm } from "@tanstack/react-form";
import { open } from "@tauri-apps/plugin-dialog";
import { Brain, FolderPen, SquareEqual, X } from "lucide-react";
import { useEffect, useState } from "react";
import { addRoot } from "@/lib/vecdir/roots/createRoot";
import { getRootsBySpaceId } from "@/lib/vecdir/roots/getRoot";
import { useAppState } from "@/store/store";
import { Button } from "../ui/button";
import { Card, CardContent, CardFooter } from "../ui/card";
import { Input } from "../ui/input";
import { Label } from "../ui/label";

interface EditSpaceParams {
    name: string;
    description: string;

    llmConfig: LLMConfig;
    embeddingConfig: EmbeddingConfig;
}

export function Settings() {
    const [roots, setRoots] = useState<IndexedRoot[]>([]);

    const selectedSpace = useAppState(state => state.selectedSpace);

    async function updateRoots(selectedSpaceId: number, set: (r: IndexedRoot[]) => void): Promise<void> {
        const newRoots = await getRootsBySpaceId(selectedSpaceId);

        set(newRoots);
    }

    useEffect(() => {
        updateRoots(selectedSpace, setRoots).then(() => {});
    }, [selectedSpace]);

    const defaultValues: EditSpaceParams = {
        name: "default",
        description: "default space",

        llmConfig: {
            model: "mistralai/ministral-3-3b",
            system_prompt: "you are a cool RAG decription tool",
            user_prompt: "describe this image",
        },

        embeddingConfig: {
            model: "text-embedding-qwen3-embedding-0.6b",
            dimensions: 768,
        },
    };
    const form = useForm({
        defaultValues,

        validators: { onChange: ({ value }) => !value ? "This field is required" : undefined },

        onSubmit: async ({ value }) => {
            // const createdSpace = await createSpace(value.name, value.llmConfig, value.embeddingConfig);

            // if (createdSpace === undefined) {
            //     // TODO: handle this exception
            //     console.log("failed to create a new space");
            //     return;
            // }

            // addSpaceToStore(createdSpace);

            // navigate({ to: "/" });
        },
    });

    return (
        <div className="p-8 max-w-4xl mx-auto">
            <div className="mb-8">
                <h2 className="text-3xl font-bold mb-2">Directories</h2>
                <p>Directories to index</p>
            </div>
            <Card className="p-6 bg-card border-border">
                <CardContent>
                    <ul className="space-y-2">
                        {
                            roots.map(root => (
                                <li
                                    key={root.id}
                                    className="flex items-center justify-between p-2 rounded-md border bg-card hover:bg-accent/50 transition-colors"
                                >
                                    <span>{root.path}</span>
                                    <Button
                                        variant="ghost"
                                        size="icon"
                                        className="h-8 w-8 text-muted-foreground hover:text-destructive"
                                        onClick={() => {}}
                                    >
                                        <X className="h-4 w-4" />
                                    </Button>
                                </li>
                            ))
                        }
                    </ul>
                </CardContent>
                <CardFooter>
                    <Button
                        variant="outline"
                        className="w-full"
                        onClick={async () => {
                            const path = await open({
                                multiple: false,
                                directory: true,
                            });

                            if (!path) {
                                return;
                            }

                            await addRoot(selectedSpace, path);

                            await updateRoots(selectedSpace, setRoots);
                        }}
                    >
                        Add Directory
                    </Button>
                </CardFooter>
            </Card>
            <div className="mb-8 mt-8">
                <h2 className="text-3xl font-bold mb-2">Edit this Space</h2>
            </div>
            <Card className="p-6 bg-card border-border">
                <form
                    onSubmit={(e) => {
                        e.preventDefault();
                        e.stopPropagation();
                        form.handleSubmit();
                    }}
                    className="space-y-6"
                >
                    <div className="space-y-6">
                        <div className="flex items-center gap-2 mb-4">
                            <FolderPen className="h-5 w-5 text-primary" />
                            <h3 className="text-xl font-semibold">Space</h3>
                        </div>
                        <form.Field name="name" validators={{ onChange: ({ value }) => !value ? "This field is required!" : undefined }}>
                            {field => (
                                <div className="space-y-2">
                                    <Label htmlFor={field.name}>Name</Label>
                                    <Input
                                        id={field.name}
                                        name={field.name}
                                        value={field.state.value}
                                        onBlur={field.handleBlur}
                                        onChange={e => field.handleChange(e.target.value)}
                                        placeholder="Name this Space"
                                        className="border-border"
                                    />
                                </div>
                            )}
                        </form.Field>
                        <form.Field name="description" validators={{ onChange: ({ value }) => !value ? "This field is required!" : undefined }}>
                            {field => (
                                <div className="space-y-2">
                                    <Label htmlFor={field.name}>Description</Label>
                                    <Input
                                        id={field.name}
                                        name={field.name}
                                        value={field.state.value}
                                        onBlur={field.handleBlur}
                                        onChange={e => field.handleChange(e.target.value)}
                                        placeholder="Describe this Space"
                                        className="border-border"
                                    />
                                </div>
                            )}
                        </form.Field>
                        <div className="flex items-center gap-2 mb-4">
                            <Brain className="h-5 w-5 text-primary" />
                            <h3 className="text-xl font-semibold">LLM Settings</h3>
                        </div>
                        <form.Field name="llmConfig.model" validators={{ onChange: ({ value }) => !value ? "This field is required!" : undefined }}>
                            {field => (
                                <div className="space-y-2">
                                    <Label htmlFor={field.name}>Model</Label>
                                    <Input
                                        id={field.name}
                                        name={field.name}
                                        value={field.state.value}
                                        onBlur={field.handleBlur}
                                        onChange={e => field.handleChange(e.target.value)}
                                        placeholder="Model name of your LLM"
                                        className="border-border"
                                    />
                                </div>
                            )}
                        </form.Field>
                        <form.Field name="llmConfig.system_prompt" validators={{ onChange: ({ value }) => !value ? "This field is required!" : undefined }}>
                            {field => (
                                <div className="space-y-2">
                                    <Label htmlFor={field.name}>System Prompt</Label>
                                    <Input
                                        id={field.name}
                                        name={field.name}
                                        value={field.state.value}
                                        onBlur={field.handleBlur}
                                        onChange={e => field.handleChange(e.target.value)}
                                        placeholder="System Prompt"
                                        className="border-border"
                                    />
                                </div>
                            )}
                        </form.Field>
                        <form.Field name="llmConfig.user_prompt" validators={{ onChange: ({ value }) => !value ? "This field is required!" : undefined }}>
                            {field => (
                                <div className="space-y-2">
                                    <Label htmlFor={field.name}>User Prompt</Label>
                                    <Input
                                        id={field.name}
                                        name={field.name}
                                        value={field.state.value}
                                        onBlur={field.handleBlur}
                                        onChange={e => field.handleChange(e.target.value)}
                                        placeholder="User Prompt"
                                        className="border-border"
                                    />
                                </div>
                            )}
                        </form.Field>
                        <div className="flex items-center gap-2 mb-4">
                            <SquareEqual className="h-5 w-5 text-primary" />
                            <h3 className="text-xl font-semibold">Embedding Settings</h3>
                        </div>
                        <form.Field name="embeddingConfig.model" validators={{ onChange: ({ value }) => !value ? "This field is required!" : undefined }}>
                            {field => (
                                <div className="space-y-2">
                                    <Label htmlFor={field.name}>Embedding Model</Label>
                                    <Input
                                        id={field.name}
                                        name={field.name}
                                        value={field.state.value}
                                        onBlur={field.handleBlur}
                                        onChange={e => field.handleChange(e.target.value)}
                                        placeholder="Embedding Model"
                                        className="border-border"
                                        disabled={true}
                                    />
                                </div>
                            )}
                        </form.Field>
                    </div>
                    <form.Subscribe
                        selector={state => [state.canSubmit, state.isSubmitting]}
                    >
                        {([canSubmit, isSubmitting]) => (
                            <Button
                                type="submit"
                                className="w-full"
                                disabled={!canSubmit}
                            >
                                {isSubmitting ? "Updating this Space" : "Update this Space"}
                            </Button>
                        )}
                    </form.Subscribe>
                </form>
            </Card>
        </div>
    );
}
