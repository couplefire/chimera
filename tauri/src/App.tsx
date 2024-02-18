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
  const [activeResult, setActiveResult] = useState<number>(0);

  useEffect(() => {
    window.addEventListener('click', (e: MouseEvent) => {
      if (!document.getElementById('app')?.contains(e.target as Node)) {
        appWindow.hide();
      }
    });

    window.addEventListener('keydown', (e: KeyboardEvent) => {
      if (e.key === 'ArrowDown') {
        setActiveResult((activeResult + 1) % searchResults.length);
      } else if (e.key === 'ArrowUp') {
        setActiveResult((activeResult - 1 + searchResults.length) % searchResults.length);
      }
    });

    return () => {
      window.removeEventListener('click', () => {
        appWindow.hide();
      });
      window.removeEventListener('keydown', () => {
        setActiveResult(0);
      });
    }
  }, [])

  const handleSearchChange = async (text: string) => {
    const res = await invoke("search", { searchText: text});
    if (res) {
      const searchResults = res as SearchResult[];
      setSearchResults(searchResults);
      setActiveResult(0);
    };
  };

  const debouncedSearch = debounce(handleSearchChange, 100, { trailing: true, leading: true })

  return (
    <div id="app">
      <SearchBox onSearchChange={debouncedSearch} />
      <div className="results">
        {searchResults.map((result, index) => (
          <ResultBox key={index} isActive={index === activeResult} result={result} />
        ))}
      </div>
    </div>
  );
}

export default App;
