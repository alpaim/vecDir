export interface SearchParams {
    searchQuery: string;
}

export function Search({ searchQuery }: SearchParams) {
    return (
        <div>
            Your query:
            {" "}
            {searchQuery}
        </div>
    );
}
