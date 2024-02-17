import { ChangeEvent } from 'react';


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
    />
  );
}

export default SearchBox;