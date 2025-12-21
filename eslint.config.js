import antfu from "@antfu/eslint-config";

export default antfu({
    stylistic: {
        semi: true,
        indent: 4,
        quotes: "double",
    },
    typescript: {
        overrides: {
            "no-console": "off", // not sure if I should keep it for further development iterations
        },
    },
    rules: {
        "node/prefer-global/process": "off",
    },
    ignores: [
        "**/lib/vecdir/bindings.ts",
    ],
});
