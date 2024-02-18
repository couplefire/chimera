import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import SearchBox from "./components/searchbox";
import ResultBox from "./components/resultbox";
import { SearchResult } from "./types/types";
import { debounce } from 'lodash'
import "./App.css";

function App() {
  const [searchResults, setSearchResults] = useState<SearchResult[]>([]);

  const handleSearchChange = async (text: string) => {
    const res = await invoke("search", { searchText: text});
    if (res) {
      const searchResults = res as SearchResult[];
      setSearchResults(searchResults);
    };
  };

  const debouncedSearch = debounce(handleSearchChange, 100, { trailing: true, leading: true })

  return (
    <div className="container">
      <SearchBox onSearchChange={debouncedSearch} />
      <div className="results">
        {searchResults.map((result, index) => (
          <ResultBox key={index} result={result} />
        ))}
      </div>
    </div>
  );
}

export default App;
