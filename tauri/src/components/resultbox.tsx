import { SearchResult } from "../types/types";
import { getLogoByExtension } from "../utils/logoutils";
import styles from "../styles/resultbox.module.css";

interface ResultBoxProps {
  result: SearchResult;
}


function SearchBox({ result }: ResultBoxProps ) {
    const file_extension = result.directory.split('.').pop(); 
    const logo_path = getLogoByExtension(file_extension);

    const renderDirectory = (directory: string) => {
        const maxDisplayLength = 20;
        const parts = directory.split('/');
        let currentPath = parts.pop() || '';
        while (parts.length > 0 && currentPath.length + parts[parts.length-1].length + 1 <= maxDisplayLength) {
            currentPath = parts.pop() + '/' + currentPath;
        }
        if (currentPath.length > maxDisplayLength) {
            currentPath = currentPath.slice(0, maxDisplayLength - 3) + '...';
        } else if (parts.length > 0) {
            currentPath = '...' + '/' + currentPath;
        }
        return currentPath;
    }

    const renderFileSize = (fileSize: number) => {
        if (fileSize < 1024 / 2) {
            return `${fileSize} bytes`;
        } else if (fileSize < 1024 * 1024 / 2) {
            return `${(fileSize / 1024).toFixed(2)} KB`;
        } else if (fileSize < 1024 * 1024 * 1024 / 2) {
            return `${(fileSize / 1024 / 1024).toFixed(2)} MB`;
        } else {
            return `${(fileSize / 1024 / 1024 / 1024).toFixed(2)} GB`;
        }
    }

    return (
        <div className={styles.container}>
            <img src={logo_path} className={styles.logo} />
            <div className={styles.textContainer}>
                <div className={styles.topLevel}>
                    <div>{result.fileName}</div>
                </div>
                <div className={styles.bottomLevel}>
                    <div>{renderDirectory(result.directory)}</div>
                    {result.numPages && 
                        <div>{result.numPages} pages</div>
                    }
                    <div>{renderFileSize(result.fileSize)}</div>
                </div>
            </div>
        </div>
    );
}

export default SearchBox;
