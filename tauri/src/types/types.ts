
export interface SearchResult {
    fileName: string;
    directory: string;
}
  
export interface SearchResponse {
    searchResults: SearchResult[];
}