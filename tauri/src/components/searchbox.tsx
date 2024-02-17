import { ChangeEvent } from 'react';
import styles from '../styles/searchbox.module.css';

interface SearchBoxProps {
    onSearchChange: (value: string) => void;
}

function SearchBox({ onSearchChange }: SearchBoxProps ) {
  // Handles input change and calls the parent's onSearchChange
  const handleChange = (event: ChangeEvent<HTMLInputElement>) => {
    onSearchChange(event.target.value);
  };

  return (
    <input
      type="text"
      placeholder="Search by what the file is about"
      onChange={handleChange}
      className={styles.searchBox}
    />
  );
}

export default SearchBox;