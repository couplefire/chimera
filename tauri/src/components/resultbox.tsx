import { SearchResult } from "../types/types";
import { getLogoByExtension } from "../utils/logoutils";

interface ResultBoxProps {
    result: SearchResult; 
}


function SearchBox({ result }: ResultBoxProps ) {
    const file_extension = result.directory.split('.').pop(); 
    const logo_path = getLogoByExtension(file_extension);
    return (
        <div className="result">
            <img src={logo_path} />
            <div className="filename">{result.fileName}</div>
            <div className="filepath">{result.directory}</div>
        </div>
    );
}

export default SearchBox;