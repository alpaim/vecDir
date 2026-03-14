import type { EmbeddingBackendType, EmbeddingConfig, IndexedRoot, LLMConfig } from "@/lib/vecdir/bindings";
import { useForm } from "@tanstack/react-form";
import { open } from "@tauri-apps/plugin-dialog";
import { Brain, FileImage, FileType, FolderPen, SquaresIntersect, X } from "lucide-react";
import { useEffect, useState } from "react";
import { toast } from "sonner";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardFooter } from "@/components/ui/card";
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogTitle } from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { addRoot } from "@/lib/vecdir/roots/createRoot";
import { deleteRoot } from "@/lib/vecdir/roots/deleteRoot";
import { getRootsBySpaceId } from "@/lib/vecdir/roots/getRoot";
import { getAllSpaces, getSpaceById } from "@/lib/vecdir/spaces/getSpace";
import { updateSpace } from "@/lib/vecdir/spaces/updateSpace";
import { useAppState } from "@/store/store";

interface EditSpaceParams {
    name: string;
    description: string | undefined;

    llmConfig: LLMConfig;
    embeddingConfig: EmbeddingConfig;
}

export function Settings() {
    const [roots, setRoots] = useState<IndexedRoot[]>([]);
    const [defaultValues, setDefaultValues] = useState<EditSpaceParams | undefined>();
    const [showBackendWarning, setShowBackendWarning] = useState(false);
    const [pendingBackend, setPendingBackend] = useState<EmbeddingBackendType | null>(null);

    const selectedSpace = useAppState(state => state.selectedSpace);

    const setSpaces = useAppState(state => state.setSpaces);

    async function updateRoots(selectedSpaceId: number, set: (r: IndexedRoot[]) => void): Promise<void> {
        const newRoots = await getRootsBySpaceId(selectedSpaceId);

        set(newRoots);
    }

    useEffect(() => {
        updateRoots(selectedSpace, setRoots).then(() => {});
    }, [selectedSpace]);

    useEffect(() => {
        getSpaceById(selectedSpace).then((space) => {
            if (space === undefined) {
                return;
            }

            const values: EditSpaceParams = {
                name: space.name,
                description: space.description ? space.description : undefined,

                llmConfig: space.llm_config,
                embeddingConfig: space.embedding_config,
            };

            setDefaultValues(values);
        });
    }, [selectedSpace]);

    const form = useForm({
        defaultValues,

        validators: { onChange: ({ value }) => !value ? "This field is required" : undefined },

        onSubmit: async ({ value }) => {
            const result = await updateSpace(
                selectedSpace,
                value.name,
                value.description || "",
                value.llmConfig,
                value.embeddingConfig,
            );

            if (result === false) {
                // TODO: handle this exception
                console.log("failed to update space");
            }

            const spaces = await getAllSpaces();

            setSpaces(spaces);

            toast("Space updated");
        },
    });

    return (
        <div className="p-8 max-w-4xl mx-auto">
            <div className="mb-8">
                <h2 className="text-3xl font-bold mb-2">Directories</h2>
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
                                        onClick={async () => {
                                            await deleteRoot(root.id);
                                            await updateRoots(selectedSpace, setRoots);
                                        }}
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
                        variant="default"
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
                            <SquaresIntersect className="h-5 w-5 text-primary" />
                            <h3 className="text-xl font-semibold">Embedding Settings</h3>
                        </div>
                        <form.Field name="embeddingConfig.backend">
                            {field => (
                                <div className="space-y-2">
                                    <Label htmlFor={field.name}>Backend Type</Label>
                                    <Select
                                        value={field.state.value ?? "openai_compat"}
                                        onValueChange={value => {
                                            if (value !== field.state.value) {
                                                setPendingBackend(value as EmbeddingBackendType);
                                                setShowBackendWarning(true);
                                            }
                                        }}
                                    >
                                        <SelectTrigger id={field.name}>
                                            <SelectValue placeholder="Select backend" />
                                        </SelectTrigger>
                                        <SelectContent>
                                            <SelectItem value="openai_compat">OpenAI Compatible (LLM + Embedding)</SelectItem>
                                            <SelectItem value="vecbox">VecBox (Multimodal Embeddings)</SelectItem>
                                        </SelectContent>
                                    </Select>
                                </div>
                            )}
                        </form.Field>
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
                                    />
                                </div>
                            )}
                        </form.Field>
                        <form.Field name="embeddingConfig.api_base_url" validators={{ onChange: ({ value }) => !value ? "This field is required!" : undefined }}>
                            {field => (
                                <div className="space-y-2">
                                    <Label htmlFor={field.name}>API Base URL</Label>
                                    <Input
                                        id={field.name}
                                        name={field.name}
                                        value={field.state.value}
                                        onBlur={field.handleBlur}
                                        onChange={e => field.handleChange(e.target.value)}
                                        placeholder="API Base URL"
                                        className="border-border"
                                    />
                                </div>
                            )}
                        </form.Field>
                        <form.Field name="embeddingConfig.api_key" validators={{ onChange: ({ value }) => !value ? "This field is required!" : undefined }}>
                            {field => (
                                <div className="space-y-2">
                                    <Label htmlFor={field.name}>API Key</Label>
                                    <Input
                                        id={field.name}
                                        name={field.name}
                                        value={field.state.value}
                                        onBlur={field.handleBlur}
                                        onChange={e => field.handleChange(e.target.value)}
                                        placeholder="API Key"
                                        className="border-border"
                                    />
                                </div>
                            )}
                        </form.Field>
                        <form.Subscribe selector={state => state.values.embeddingConfig?.backend}>
                            {(backend) => {
                                if (backend === "vecbox")
                                    return null;
                                return (
                                    <>
                                        <div className="flex items-center gap-2 mb-4 mt-6">
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
                                        <form.Field name="llmConfig.api_base_url" validators={{ onChange: ({ value }) => !value ? "This field is required!" : undefined }}>
                                            {field => (
                                                <div className="space-y-2">
                                                    <Label htmlFor={field.name}>API Base URL</Label>
                                                    <Input
                                                        id={field.name}
                                                        name={field.name}
                                                        value={field.state.value}
                                                        onBlur={field.handleBlur}
                                                        onChange={e => field.handleChange(e.target.value)}
                                                        placeholder="API Base URL"
                                                        className="border-border"
                                                    />
                                                </div>
                                            )}
                                        </form.Field>
                                        <form.Field name="llmConfig.api_key" validators={{ onChange: ({ value }) => !value ? "This field is required!" : undefined }}>
                                            {field => (
                                                <div className="space-y-2">
                                                    <Label htmlFor={field.name}>API Key</Label>
                                                    <Input
                                                        id={field.name}
                                                        name={field.name}
                                                        value={field.state.value}
                                                        onBlur={field.handleBlur}
                                                        onChange={e => field.handleChange(e.target.value)}
                                                        placeholder="API Key"
                                                        className="border-border"
                                                    />
                                                </div>
                                            )}
                                        </form.Field>
                                        <Card className="p-5">
                                            <div className="flex items-center gap-2 mb-4">
                                                <FileType />
                                                <h3 className="text-l font-semibold">Text Processing Prompt</h3>
                                            </div>
                                            <form.Field name="llmConfig.text_processing_prompt.system_prompt" validators={{ onChange: ({ value }) => !value ? "This field is required!" : undefined }}>
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
                                            <form.Field name="llmConfig.text_processing_prompt.user_prompt" validators={{ onChange: ({ value }) => !value ? "This field is required!" : undefined }}>
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
                                        </Card>
                                        <Card className="p-5">
                                            <div className="flex items-center gap-2 mb-4">
                                                <FileImage />
                                                <h3 className="text-l font-semibold">Image Processing Prompt</h3>
                                            </div>
                                            <form.Field name="llmConfig.image_processing_prompt.system_prompt" validators={{ onChange: ({ value }) => !value ? "This field is required!" : undefined }}>
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
                                            <form.Field name="llmConfig.image_processing_prompt.user_prompt" validators={{ onChange: ({ value }) => !value ? "This field is required!" : undefined }}>
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
                                        </Card>
                                        <Card className="p-5">
                                            <div className="flex items-center gap-2 mb-4">
                                                <FileType />
                                                <h3 className="text-l font-semibold">Default Processing Prompt</h3>
                                            </div>
                                            <form.Field name="llmConfig.default_processing_prompt.system_prompt" validators={{ onChange: ({ value }) => !value ? "This field is required!" : undefined }}>
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
                                            <form.Field name="llmConfig.default_processing_prompt.user_prompt" validators={{ onChange: ({ value }) => !value ? "This field is required!" : undefined }}>
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
                                        </Card>
                                    </>
                                );
                            }}
                        </form.Subscribe>
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
                <Dialog open={showBackendWarning} onOpenChange={setShowBackendWarning}>
                    <DialogContent>
                        <DialogTitle>Change Embedding Backend?</DialogTitle>
                        <DialogDescription>
                            Changing the embedding backend will most likely make existing embeddings incompatible.
                            You may need to re-index your files for search to work correctly.
                        </DialogDescription>
                        <DialogFooter>
                            <Button variant="outline" onClick={() => setShowBackendWarning(false)}>Cancel</Button>
                            <Button onClick={() => {
                                if (pendingBackend) {
                                    form.setFieldValue("embeddingConfig.backend", pendingBackend);
                                }
                                setShowBackendWarning(false);
                            }}>Continue</Button>
                        </DialogFooter>
                    </DialogContent>
                </Dialog>
            </Card>
        </div>
    );
}
