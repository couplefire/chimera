export interface SearchResult {
  filename: string;
  directory: string;
}

export interface SearchResponse {
  searchResults: SearchResult[];
}
