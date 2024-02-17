import { useState, useRef } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import SearchBox from "./components/searchbox";
import ResultBox from "./components/resultbox";
import { SearchResult, SearchResponse } from "./types/types";
import "./App.css";

function App() {
  const [searchText, setSearchText] = useState<string>('');
  const [searchResults, setSearchResults] = useState<SearchResult[]>([]);
  const timerRef = useRef<number | null>(null);

  const handleSearchChange = async (text: string) => {
    setSearchText(text);
    if (timerRef.current !== null) {
      clearTimeout(timerRef.current);
    }


    timerRef.current = window.setTimeout(async () => {
      const res = await invoke("search", { searchText });
      if (res) {
        const { searchResults } = res as SearchResponse;
        setSearchResults(searchResults);
      }
    }, 100);
  };

  return (
    <div className="container">
      <SearchBox onSearchChange={handleSearchChange} />
      <div className="results">
        {searchResults.map((result, index) => (
          <ResultBox key={index} result={result} />
        ))}
      </div>
    </div>
  );
}

export default App;
