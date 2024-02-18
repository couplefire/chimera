export interface SearchResult {
    fileName: string;
    directory: string;
    fileSize: number;
    numPages: number | null;
}

export interface SearchResponse {
  searchResults: SearchResult[];
}
