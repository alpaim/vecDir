export function isImageFile(path: string): boolean {
    const imageExtensions = [
        ".jpg",
        ".jpeg",
        ".png",
        ".gif",
        ".bmp",
        ".webp",
        ".svg",
        ".ico",
    ];
    const ext = path.toLowerCase().split(".").pop();
    return ext !== undefined && imageExtensions.includes(`.${ext}`);
}
