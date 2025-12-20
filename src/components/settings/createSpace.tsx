import type { EmbeddingConfig, LLMConfig } from "@/lib/vecdir/bindings";

import { useForm } from "@tanstack/react-form";
import { useNavigate } from "@tanstack/react-router";
import { Brain, FileImage, FileType, FolderPen, SquaresIntersect, Text } from "lucide-react";
import { toast } from "sonner";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { createSpace } from "@/lib/vecdir/spaces/createSpace";
import { useAppState } from "@/store/store";

interface CreateSpaceParams {
    name: string;
    description: string;

    llmConfig: LLMConfig;
    embeddingConfig: EmbeddingConfig;
}

export function CreateSpace() {
    const addSpaceToStore = useAppState(state => state.addSpace);
    const navigate = useNavigate({ from: "/createSpace" });

    const defaultValues: CreateSpaceParams = {
        name: "default",
        description: "default space",

        llmConfig: {
            model: "mistralai/ministral-3-3b",

            text_processing_prompt: {
                system_prompt: "you are a text processing LLM for RAG purposes",
                user_prompt: "describe this text file",
            },
            image_processing_prompt: {
                system_prompt: "you are an image descriptor. you describe images very precisely to use these descriptions in RAG",
                user_prompt: "describe this image",
            },
            default_processing_prompt: {
                system_prompt: "you are a file processing LLM for RAG purposes",
                user_prompt: "describe this file based on its metadata: ",
            },

            api_base_url: "http://127.0.0.1:1234/v1", // LM Studio
            api_key: "lmstudio",
        },

        embeddingConfig: {
            model: "text-embedding-qwen3-embedding-0.6b",
            dimensions: 768,

            api_base_url: "http://127.0.0.1:1234/v1", // LM Studio
            api_key: "lmstudio",
        },
    };
    const form = useForm({
        defaultValues,

        validators: { onChange: ({ value }) => !value ? "This field is required" : undefined },

        onSubmit: async ({ value }) => {
            const createdSpace = await createSpace(value.name, value.description, value.llmConfig, value.embeddingConfig);

            if (createdSpace === undefined) {
                // TODO: handle this exception
                console.log("failed to create a new space");
                return;
            }

            toast("Space Created");

            addSpaceToStore(createdSpace);

            navigate({ to: "/" });
        },
    });
    return (
        <div className="p-8 max-w-4xl mx-auto">
            <div className="mb-8">
                <h2 className="text-3xl font-bold mb-2">Create new Space</h2>
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
                                <FileImage />
                                <h3 className="text-l font-semibold">Default Processing Prompt </h3>
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
                        <div className="flex items-center gap-2 mb-4">
                            <SquaresIntersect className="h-5 w-5 text-primary" />
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
                                        placeholder="Base URL"
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
                                {isSubmitting ? "Creating new Space" : "Create new Space"}
                            </Button>
                        )}
                    </form.Subscribe>

                </form>
            </Card>
        </div>
    );
}
