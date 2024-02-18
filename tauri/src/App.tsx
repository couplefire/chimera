import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow } from "@tauri-apps/api/window";
import SearchBox from "./components/searchbox";
import ResultBox from "./components/resultbox";
import { SearchResult } from "./types/types";
import { debounce } from 'lodash'
import "./App.css";

function App() {
  const [searchResults, setSearchResults] = useState<SearchResult[]>([]);

  useEffect(() => {
    window.addEventListener('click', (e: MouseEvent) => {
      if (!document.getElementById('app')?.contains(e.target as Node)) {
        appWindow.hide();
      }
    });

    return () => {
      window.removeEventListener('click', () => {
        appWindow.hide();
      });
    }
  }, [])

  const handleSearchChange = async (text: string) => {
    const res = await invoke("search", { searchText: text});
    if (res) {
      const searchResults = res as SearchResult[];
      setSearchResults(searchResults);
    };
  };

  const debouncedSearch = debounce(handleSearchChange, 100, { trailing: true, leading: true })

  return (
    <div id="app">
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
